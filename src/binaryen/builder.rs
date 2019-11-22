use super::sys;

use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::path::Path;

pub struct Module {
    // TODO: make this private
    pub module: sys::BinaryenModuleRef,
}

impl Module {
    pub fn new() -> Module {
        Module { module: unsafe { sys::BinaryenModuleCreate() } }
    }

    pub fn auto_drop(&mut self) {
        // TODO: it'd be nice not to have to use this
        unsafe {
            sys::BinaryenModuleAutoDrop(self.module);
        }
    }

    pub fn is_valid(&mut self) -> bool {
        unsafe { sys::BinaryenModuleValidate(self.module) == 1 }
    }

    pub fn optimize(&mut self) {
        unsafe { sys::BinaryenModuleOptimize(self.module) }
    }

    pub fn create_func(&mut self) -> Fn {
        Fn {
            module: self,
            vars: Vec::new(),
            num_args: 0,
            has_locals: false,
        }
    }

    pub fn create_function_type<'module>(&'module mut self,
                                         name: &'module CString,
                                         arg_tys: &[ReprType],
                                         ret_ty: Type)
                                         -> FnType {
        let arg_tys: Vec<_> = arg_tys.iter().map(sys::BinaryenType::from).collect();
        let ty = unsafe {
            sys::BinaryenAddFunctionType(self.module,
                                         name.as_ptr(),
                                         ret_ty.into(),
                                         arg_tys.as_ptr(),
                                         arg_tys.len().into())
        };
        FnType {
            module: self,
            name: name,
            raw_arg_tys: arg_tys,
            type_ref: ty,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        unsafe {
            // TODO: find a way to determine the size of the buffer first. Right now we just make a
            // 4MB buffer and truncate.
            let mut buffer = Vec::with_capacity(1 << 22);
            let size = sys::BinaryenModuleWrite(self.module,
                                                mem::transmute(buffer.as_mut_ptr()),
                                                buffer.capacity());

            buffer.set_len(size);
            buffer.shrink_to_fit();

            buffer
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = try!(File::create(path));
        let buffer = self.serialize();

        file.write_all(buffer.as_slice())
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { sys::BinaryenModuleDispose(self.module) };
    }
}

pub struct FnType<'module, 'name> {
    module: &'module Module,
    name: &'name CString,
    raw_arg_tys: Vec<sys::BinaryenType>,
    type_ref: sys::BinaryenFunctionTypeRef,
}

impl<'a, 'module, 'name> From<&'a FnType<'module, 'name>> for sys::BinaryenFunctionTypeRef {
    fn from(ty: &FnType<'module, 'name>) -> sys::BinaryenFunctionTypeRef {
        ty.type_ref
    }
}

impl<'module, 'name> From<FnType<'module, 'name>> for sys::BinaryenFunctionTypeRef {
    fn from(ty: FnType<'module, 'name>) -> sys::BinaryenFunctionTypeRef {
        ty.type_ref
    }
}

/// Representable types.
///
/// These are types that can actually exist, for example, as a local variable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReprType {
    Int32,
    Int64,
    Float32,
    Float64,
}

impl<'a> From<&'a ReprType> for sys::BinaryenType {
    fn from(ty: &ReprType) -> sys::BinaryenType {
        match ty {
            &ReprType::Int32 => sys::BinaryenInt32(),
            &ReprType::Int64 => sys::BinaryenInt64(),
            &ReprType::Float32 => sys::BinaryenFloat32(),
            &ReprType::Float64 => sys::BinaryenFloat64(),
        }
    }
}

impl From<ReprType> for sys::BinaryenType {
    fn from(ty: ReprType) -> sys::BinaryenType {
        sys::BinaryenType::from(&ty)
    }
}

impl<'a> From<&'a Type> for sys::BinaryenType {
    fn from(ty: &Type) -> sys::BinaryenType {
        match *ty {
            None => sys::BinaryenNone(),
            Some(ref ty) => sys::BinaryenType::from(ty),
        }
    }
}

impl From<Type> for sys::BinaryenType {
    fn from(ty: Type) -> sys::BinaryenType {
        sys::BinaryenType::from(&ty)
    }
}

/// Any type that binaryen supports.
pub type Type = Option<ReprType>;

pub struct Fn<'module> {
    // TODO: does this need to be mutable?
    // TODO: this should not be public
    pub module: &'module mut Module,

    vars: Vec<Var>,
    num_args: usize,
    has_locals: bool,
}

impl<'module> Fn<'module> {
    pub fn add_arg(&mut self, ty: ReprType) -> &Var {
        assert!(!self.has_locals);
        self.num_args += 1;
        self.create_local_raw(ty)
    }

    pub fn create_local(&mut self, ty: ReprType) -> &Var {
        self.has_locals = true;
        self.create_local_raw(ty)
    }

    fn create_local_raw(&mut self, ty: ReprType) -> &Var {
        let index = self.vars.len();
        let var = Var {
            ty: ty,
            index: index,
        };
        self.vars.push(var);
        self.vars.last().unwrap()
    }

    // TODO: this should be private
    pub fn binaryen_var_types(&self) -> Vec<sys::BinaryenType> {
        self.vars[self.num_args..].iter().map(|x| x.into()).collect()
    }

    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }
    pub fn num_locals(&self) -> usize {
        self.vars.len() - self.num_args
    }

    pub fn get_var(&self, index: usize) -> &Var {
        &self.vars[index]
    }

    // TODO: ret_ty should not be a parameter here.
    pub fn create_sig_type(&mut self,
                           name: &CString,
                           ret_ty: Type)
                           -> sys::BinaryenFunctionTypeRef {
        let arg_tys: Vec<_> = self.vars[0..self.num_args].iter().map(|x| x.ty).collect();
        self.module
            .create_function_type(&name, &arg_tys[..], ret_ty)
            .into()
    }

    pub fn module_ref(&self) -> sys::BinaryenModuleRef {
        self.module.module
    }
}

pub struct Var {
    // TODO: this func field would be nice to have, but it causes issues.
    // func: &'func Fn<'func>,
    ty: ReprType,
    index: usize,
}

impl Var {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn ty(&self) -> ReprType {
        self.ty
    }
}

impl<'a> From<&'a Var> for ReprType {
    fn from(x: &Var) -> ReprType {
        x.ty
    }
}

impl From<Var> for ReprType {
    fn from(x: Var) -> ReprType {
        ReprType::from(&x)
    }
}

// TODO: see if we can do this for T: Into<u32>
impl From<usize> for sys::BinaryenIndex {
    fn from(i: usize) -> sys::BinaryenIndex {
        // TODO: make sure i fits in a u32
        sys::BinaryenIndex(i as u32)
    }
}

impl From<u32> for sys::BinaryenIndex {
    fn from(i: u32) -> sys::BinaryenIndex {
        sys::BinaryenIndex(i)
    }
}

impl From<Var> for sys::BinaryenIndex {
    fn from(i: Var) -> sys::BinaryenIndex {
        i.index().into()
    }
}

impl<'a> From<&'a Var> for sys::BinaryenIndex {
    fn from(i: &Var) -> sys::BinaryenIndex {
        i.index().into()
    }
}

impl<'a> From<&'a Var> for sys::BinaryenType {
    fn from(i: &Var) -> sys::BinaryenType {
        i.ty.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_module() {
        let _ = Module::new();
    }
}

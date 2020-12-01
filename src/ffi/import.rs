use crate::ffi::object::PyObject;
use std::os::raw::{c_char, c_int, c_long};

extern "C" {
    pub fn PyImport_GetMagicNumber() -> c_long;
    pub fn PyImport_GetMagicTag() -> *const c_char;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ExecCodeModule")]
    pub fn PyImport_ExecCodeModule(name: *const c_char, co: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ExecCodeModuleEx")]
    pub fn PyImport_ExecCodeModuleEx(
        name: *const c_char,
        co: *mut PyObject,
        pathname: *const c_char,
    ) -> *mut PyObject;
    pub fn PyImport_ExecCodeModuleWithPathnames(
        name: *const c_char,
        co: *mut PyObject,
        pathname: *const c_char,
        cpathname: *const c_char,
    ) -> *mut PyObject;
    pub fn PyImport_ExecCodeModuleObject(
        name: *mut PyObject,
        co: *mut PyObject,
        pathname: *mut PyObject,
        cpathname: *mut PyObject,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_GetModuleDict")]
    pub fn PyImport_GetModuleDict() -> *mut PyObject;
    pub fn PyImport_AddModuleObject(name: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_AddModule")]
    pub fn PyImport_AddModule(name: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModule")]
    pub fn PyImport_ImportModule(name: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModuleNoBlock")]
    pub fn PyImport_ImportModuleNoBlock(name: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModuleLevel")]
    pub fn PyImport_ImportModuleLevel(
        name: *const c_char,
        globals: *mut PyObject,
        locals: *mut PyObject,
        fromlist: *mut PyObject,
        level: c_int,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ImportModuleLevelObject")]
    pub fn PyImport_ImportModuleLevelObject(
        name: *mut PyObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
        fromlist: *mut PyObject,
        level: c_int,
    ) -> *mut PyObject;
}

#[inline]
pub unsafe fn PyImport_ImportModuleEx(
    name: *const c_char,
    globals: *mut PyObject,
    locals: *mut PyObject,
    fromlist: *mut PyObject,
) -> *mut PyObject {
    PyImport_ImportModuleLevel(name, globals, locals, fromlist, 0)
}

extern "C" {
    pub fn PyImport_GetImporter(path: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_Import")]
    pub fn PyImport_Import(name: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyImport_ReloadModule")]
    pub fn PyImport_ReloadModule(m: *mut PyObject) -> *mut PyObject;
    #[cfg(not(Py_3_9))]
    pub fn PyImport_Cleanup();
    pub fn PyImport_ImportFrozenModuleObject(name: *mut PyObject) -> c_int;
    pub fn PyImport_ImportFrozenModule(name: *const c_char) -> c_int;

    pub fn PyImport_AppendInittab(
        name: *const c_char,
        initfunc: Option<extern "C" fn() -> *mut PyObject>,
    ) -> c_int;
}


#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(Py_LIMITED_API))]
pub struct _inittab {
    pub name: *mut c_char,
    pub initfunc: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(Py_LIMITED_API))]
pub struct _frozen {
    pub name: *const c_char,
    pub code: *const c_uchar,
    pub size: c_int,
}

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyImport_FrozenModules: *const _frozen;
    pub static mut PyImport_Inittab: *mut _inittab;

    pub fn PyImport_ExtendInittab(newtab: *const _inittab) -> c_int;

    #[cfg(not(Py_3_7))]
    pub fn _PyImport_FindBuiltin(name: *const c_char) -> *mut PyObject;
    #[cfg(all(Py_3_7, not(Py_3_9)))]
    pub fn _PyImport_FindBuiltin(name: *const c_char, modules: *mut PyObject) -> *mut PyObject;

    pub fn _PyImport_FindExtensionObject(
        name: *mut PyObject,
        filename: *mut PyObject,
    ) -> *mut PyObject;

    #[cfg(all(Py_3_7, not(Py_3_9)))]
    pub fn _PyImport_FindExtensionObjectEx(
        name: *mut PyObject,
        filename: *mut PyObject,
        modules: *mut PyObject,
    ) -> *mut PyObject;

    #[cfg(not(Py_3_7))]
    pub fn _PyImport_FixupBuiltin(module: *mut PyObject, name: *const c_char) -> c_int;
    #[cfg(Py_3_7)]
    pub fn _PyImport_FixupBuiltin(
        module: *mut PyObject,
        name: *const c_char,
        modules: *mut PyObject,
    ) -> c_int;

    #[cfg(not(Py_3_7))]
    pub fn _PyImport_FixupExtensionObject(
        module: *mut PyObject,
        name: *mut PyObject,
        filename: *mut PyObject,
    ) -> c_int;
    #[cfg(Py_3_7)]
    pub fn _PyImport_FixupExtensionObject(
        module: *mut PyObject,
        name: *mut PyObject,
        filename: *mut PyObject,
        modules: *mut PyObject,
    ) -> c_int;
}


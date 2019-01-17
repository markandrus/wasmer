use crate::{memory::LinearMemory, structures::TypedIndex, types::LocalMemoryIndex, vm};

pub unsafe extern "C" fn memory_grow_static(
    memory_index: LocalMemoryIndex,
    by_pages: u32,
    ctx: *mut vm::Ctx,
) -> i32 {
    if let Some(old) = (*(*ctx).local_backing)
        .memory(memory_index)
        .grow_static(by_pages)
    {
        // Store the new size back into the vmctx.
        (*(*ctx).memories.add(memory_index.index())).size =
            (old as usize + by_pages as usize) * LinearMemory::PAGE_SIZE as usize;
        old
    } else {
        -1
    }
}

pub unsafe extern "C" fn memory_size(memory_index: LocalMemoryIndex, ctx: *mut vm::Ctx) -> u32 {
    (*(*ctx).local_backing).memory(memory_index).pages()
}

pub unsafe extern "C" fn memory_grow_dynamic(
    memory_index: LocalMemoryIndex,
    by_pages: u32,
    ctx: *mut vm::Ctx,
) -> i32 {
    if let Some(old) = (*(*ctx).local_backing)
        .memory(memory_index)
        .grow_dynamic(by_pages)
    {
        // Store the new size back into the vmctx.
        (*(*ctx).memories.add(memory_index.index())).size =
            (old as usize + by_pages as usize) * LinearMemory::PAGE_SIZE as usize;
        old
    } else {
        -1
    }
}
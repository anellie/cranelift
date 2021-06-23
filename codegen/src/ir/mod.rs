//! Representation of Cranelift IR functions.

mod atomic_rmw_op;
mod builder;
pub mod constant;
pub mod dfg;
pub mod entities;
mod extfunc;
mod extname;
pub mod function;
mod globalvalue;
mod heap;
pub mod immediates;
pub mod instructions;
pub mod jumptable;
pub mod layout;
pub(crate) mod libcall;
mod memflags;
mod progpoint;
mod sourceloc;
pub mod stackslot;
mod table;
mod trapcode;
pub mod types;
mod valueloc;

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

pub use crate::{
    ir::{
        atomic_rmw_op::AtomicRmwOp,
        builder::{InsertBuilder, InstBuilder, InstBuilderBase, InstInserterBase, ReplaceBuilder},
        constant::{ConstantData, ConstantOffset, ConstantPool},
        dfg::{DataFlowGraph, ValueDef},
        entities::{
            Block, Constant, FuncRef, GlobalValue, Heap, Immediate, Inst, JumpTable, SigRef,
            StackSlot, Table, Value,
        },
        extfunc::{AbiParam, ArgumentExtension, ArgumentPurpose, ExtFuncData, Signature},
        extname::ExternalName,
        function::{DisplayFunctionAnnotations, Function},
        globalvalue::GlobalValueData,
        heap::{HeapData, HeapStyle},
        instructions::{InstructionData, Opcode, ValueList, ValueListPool, VariableArgs},
        jumptable::JumpTableData,
        layout::Layout,
        libcall::{get_probestack_funcref, LibCall},
        memflags::{Endianness, MemFlags},
        progpoint::{ExpandedProgramPoint, ProgramOrder, ProgramPoint},
        sourceloc::SourceLoc,
        stackslot::{StackLayoutInfo, StackSlotData, StackSlotKind, StackSlots},
        table::TableData,
        trapcode::TrapCode,
        types::Type,
        valueloc::{ArgumentLoc, ValueLoc},
    },
    value_label::LabelValueLoc,
};
pub use cranelift_codegen_shared::condcodes;

use crate::{
    binemit,
    entity::{entity_impl, PrimaryMap, SecondaryMap},
    isa,
};

/// Map of value locations.
pub type ValueLocations = SecondaryMap<Value, ValueLoc>;

/// Map of jump tables.
pub type JumpTables = PrimaryMap<JumpTable, JumpTableData>;

/// Map of instruction encodings.
pub type InstEncodings = SecondaryMap<Inst, isa::Encoding>;

/// Code offsets for blocks.
pub type BlockOffsets = SecondaryMap<Block, binemit::CodeOffset>;

/// Code offsets for Jump Tables.
pub type JumpTableOffsets = SecondaryMap<JumpTable, binemit::CodeOffset>;

/// Source locations for instructions.
pub type SourceLocs = SecondaryMap<Inst, SourceLoc>;

/// Marked with a label value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLabel(u32);
entity_impl!(ValueLabel, "val");

/// A label of a Value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLabelStart {
    /// Source location when it is in effect
    pub from: SourceLoc,

    /// The label index.
    pub label: ValueLabel,
}

/// Value label assignements: label starts or value aliases.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum ValueLabelAssignments {
    /// Original value labels assigned at transform.
    Starts(alloc::vec::Vec<ValueLabelStart>),

    /// A value alias to original value.
    Alias {
        /// Source location when it is in effect
        from: SourceLoc,

        /// The label index.
        value: Value,
    },
}

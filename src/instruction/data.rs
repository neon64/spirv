#![allow(non_snake_case)]

use instruction::{Id, LiteralNumber, LiteralString, UnsizedArray};
use enumerations::*;

macro_rules! instruction {
    ($struct_name:ident, $min_word_count:expr, $max_word_count:expr) => (
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct $struct_name;
    );
    ($struct_name:ident, $min_word_count:expr, $max_word_count:expr, $($name:ident: $ty:ty),*) => (
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct $struct_name {
            $(pub $name: $ty),*
        }
    )
}

macro_rules! instruction_base {
    ($struct_name:ident, $min_word_count:expr, $max_word_count:expr, $($name:ident: $ty:ty),*) => (
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct $struct_name {
            /// the <id> of the new type.
            pub result: Id,
            $(pub $name: $ty),*
        }
    );
    ($struct_name:ident, $min_word_count:expr, $max_word_count:expr) => (
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct $struct_name {
            /// the <id> of the new type.
            pub result: Id
        }
    )
}


#[derive(Debug)]
pub enum Instruction<'a> {
    Nop,
    Undef(&'a OpUndef),
    Source(&'a OpSource),
    SourceExtension(&'a OpSourceExtension),
    Name(&'a OpName),
    MemberName(&'a OpMemberName),
    String(&'a OpString),
    Line(&'a OpLine),
    Capability(&'a OpCapability),
    DecorationGroup(&'a OpDecorationGroup),
    Decorate(&'a OpDecorate),
    MemberDecorate(&'a OpMemberDecorate),
    GroupDecorate(&'a OpGroupDecorate),
    GroupMemberDecorate(&'a OpGroupMemberDecorate),
    Extension(&'a OpExtension),
    ExtInstImport(&'a OpExtInstImport),
    ExtInst(&'a OpExtInst),
    MemoryModel(&'a OpMemoryModel),
    EntryPoint(&'a OpEntryPoint),
    ExecutionMode(&'a OpExecutionMode),
    TypeVoid(&'a OpTypeVoid),
    TypeBool(&'a OpTypeBool),
    TypeInt(&'a OpTypeInt),
    TypeFloat(&'a OpTypeFloat),
    TypeVector(&'a OpTypeVector),
    TypeMatrix(&'a OpTypeMatrix),
    TypeSampler(&'a OpTypeSampler),
    TypeArray(&'a OpTypeArray),
    TypeRuntimeArray(&'a OpTypeRuntimeArray),
    TypeStruct(&'a OpTypeStruct),
    TypeOpaque(&'a OpTypeOpaque),
    TypePointer(&'a OpTypePointer),
    TypeImage(&'a OpTypeImage),
    TypeSampledImage(&'a OpTypeSampledImage),
    TypeFunction(&'a OpTypeFunction),
    TypeEvent(&'a OpTypeEvent),
    TypeDeviceEvent(&'a OpTypeDeviceEvent),
    TypeReserveId(&'a OpTypeReserveId),
    TypeQueue(&'a OpTypeQueue),
    TypePipe(&'a OpTypePipe),
    ConstantTrue(&'a OpConstantTrue),
    ConstantFalse(&'a OpConstantFalse),
    Constant(&'a OpConstant),
    ConstantComposite(&'a OpConstantComposite),
    ConstantSampler(&'a OpConstantSampler),
    ConstantNull(&'a OpConstantNull),
    SpecConstantTrue(&'a OpSpecConstantTrue),
    SpecConstantFalse(&'a OpSpecConstantFalse),
    SpecConstant(&'a OpSpecConstant),
    SpecConstantComposite(&'a OpSpecConstantComposite),
    SpecConstantOp(&'a OpSpecConstantOp),
    Variable(&'a OpVariable),
    Load(&'a OpLoad),
    Store(&'a OpStore),
    CopyMemory(&'a OpCopyMemory),
    CopyMemorySized(&'a OpCopyMemorySized),
    AccessChain(&'a OpAccessChain),
    InBoundsAccessChain(&'a OpInBoundsAccessChain),
    PtrAccessChain(&'a OpPtrAccessChain),
    ArrayLength(&'a OpArrayLength),
    ImageTexelPointer(&'a OpImageTexelPointer),
    GenericPtrMemSemantics(&'a OpGenericPtrMemSemantics),
    Function(&'a OpFunction),
    FunctionParameter(&'a OpFunctionParameter),
    FunctionEnd(&'a OpFunctionEnd),
    FunctionCall(&'a OpFunctionCall),
    SampledImage(&'a OpSampledImage),
    ImageSampleImplicitLod(&'a OpImageSampleImplicitLod),
    ImageSampleExplicitLod(&'a OpImageSampleExplicitLod),
    ImageSampleDrefImplicitLod(&'a OpImageSampleDrefImplicitLod),
    ImageSampleDrefExplicitLod(&'a OpImageSampleDrefExplicitLod),
    ImageSampleProjImplicitLod(&'a OpImageSampleProjImplicitLod),
    ImageSampleProjExplicitLod(&'a OpImageSampleProjExplicitLod),
    ImageSampleProjDrefImplicitLod(&'a OpImageSampleProjDrefImplicitLod),
    ImageSampleProjDrefExplicitLod(&'a OpImageSampleProjDrefExplicitLod),
    ImageFetch(&'a OpImageFetch),
    ImageGather(&'a OpImageGather),
    ImageDrefGather(&'a OpImageDrefGather),
    ImageRead(&'a OpImageRead),
    ImageWrite(&'a OpImageRead),
    ImageQueryDim(&'a OpImageQueryDim),
    ImageQueryFormat(&'a OpImageQueryFormat),
    ImageQueryOrder(&'a OpImageQueryOrder),
    ImageQuerySizeLod(&'a OpImageQuerySizeLod),
    ImageQuerySize(&'a OpImageQuerySize),
    ImageQueryLod(&'a OpImageQuerySize),
    ImageQueryLevels(&'a OpImageQueryLevels),
    ImageQuerySamples(&'a OpImageQuerySamples),
    ConvertFToU(&'a OpConvertFToU),
    ConvertFToS(&'a OpConvertFToS),
    ConvertSToF(&'a OpConvertSToF),
    ConvertUToF(&'a OpConvertUToF),
    UConvert(&'a OpUConvert),
    SConvert(&'a OpSConvert),
    FConvert(&'a OpFConvert),
    QuantizeToF16(&'a OpQuantizeToF16),
    ConvertPtrToU(&'a OpConvertPtrToU),
    ConvertUToPtr(&'a OpConvertUToPtr),
    PtrCastToGeneric(&'a OpPtrCastToGeneric),
    GenericCastToPtr(&'a OpGenericCastToPtr),
    Bitcast(&'a OpBitcast),
    GenericCastToPtrExplicit(&'a OpGenericCastToPtrExplicit),
    SatConvertSToU(&'a OpSatConvertSToU),
    SatConvertUToS(&'a OpSatConvertUToS),
    VectorExtractDynamic(&'a OpVectorExtractDynamic),
    VectorInsertDynamic(&'a OpVectorInsertDynamic),
    VectorShuffle(&'a OpVectorShuffle),
    CompositeConstruct(&'a OpCompositeConstruct),
    CompositeExtract(&'a OpCompositeExtract),
    CompositeInsert(&'a OpCompositeInsert),
    CopyObject(&'a OpCopyObject),
    Transpose(&'a OpTranspose),
    SNegate(&'a OpSNegate),
    FNegate(&'a OpFNegate),
    Not(&'a OpNot),
    IAdd(&'a OpIAdd),
    FAdd(&'a OpFAdd),
    ISub(&'a OpISub),
    FSub(&'a OpFSub),
    IMul(&'a OpIMul),
    FMul(&'a OpFMul),
    UDiv(&'a OpUDiv),
    SDiv(&'a OpSDiv),
    FDiv(&'a OpFDiv),
    UMod(&'a OpUMod),
    SRem(&'a OpSRem),
    SMod(&'a OpSMod),
    FRem(&'a OpFRem),
    FMod(&'a OpFMod),
    VectorTimesScalar(&'a OpVectorTimesScalar),
    MatrixTimesScalar(&'a OpMatrixTimesScalar),
    VectorTimesMatrix(&'a OpVectorTimesMatrix),
    MatrixTimesVector(&'a OpMatrixTimesVector),
    MatrixTimesMatrix(&'a OpMatrixTimesMatrix),
    OuterProduct(&'a OpOuterProduct),
    Dot(&'a OpDot),
    ShiftRightLogical(&'a OpShiftRightLogical),
    ShiftRightArithmetic(&'a OpShiftRightArithmetic),
    ShiftLeftLogical(&'a OpShiftLeftLogical),
    BitwiseOr(&'a OpBitwiseOr),
    BitwiseXor(&'a OpBitwiseXor),
    BitwiseAnd(&'a OpBitwiseAnd),
    Any(&'a OpAny),
    All(&'a OpAll),
    IsNan(&'a OpIsNan),
    IsInf(&'a OpIsInf),
    IsFinite(&'a OpIsFinite),
    IsNormal(&'a OpIsNormal),
    SignBitSet(&'a OpSignBitSet),
    LessOrGreater(&'a OpLessOrGreater),
    Ordered(&'a OpOrdered),
    Unordered(&'a OpUnordered),
    LogicalOr(&'a OpLogicalOr),
    LogicalEqual(&'a OpLogicalEqual),
    LogicalNotEqual(&'a OpLogicalNotEqual),
    LogicalAnd(&'a OpLogicalAnd),
    LogicalNot(&'a OpLogicalNot),
    Select(&'a OpSelect),
    IEqual(&'a OpIEqual),
    FOrdEqual(&'a OpFOrdEqual),
    FUnordEqual(&'a OpFUnordEqual),
    INotEqual(&'a OpINotEqual),
    FOrdNotEqual(&'a OpFOrdNotEqual),
    FUnordNotEqual(&'a OpFUnordNotEqual),
    ULessThan(&'a OpULessThan),
    SLessThan(&'a OpSLessThan),
    FOrdLessThan(&'a OpFOrdLessThan),
    FUnordLessThan(&'a OpFUnordLessThan),
    UGreaterThan(&'a OpUGreaterThan),
    SGreaterThan(&'a OpSGreaterThan),
    FOrdGreaterThan(&'a OpFOrdGreaterThan),
    FUnordGreaterThan(&'a OpFUnordGreaterThan),
    ULessThanEqual(&'a OpULessThanEqual),
    SLessThanEqual(&'a OpSLessThanEqual),
    FOrdLessThanEqual(&'a OpFOrdLessThanEqual),
    FUnordLessThanEqual(&'a OpFUnordLessThanEqual),
    UGreaterThanEqual(&'a OpUGreaterThanEqual),
    SGreaterThanEqual(&'a OpSGreaterThanEqual),
    FOrdGreaterThanEqual(&'a OpFOrdGreaterThanEqual),
    FUnordGreaterThanEqual(&'a OpFUnordGreaterThanEqual),
    DPdx(&'a OpDPdx),
    DPdy(&'a OpDPdy),
    Fwidth(&'a OpFwidth),
    DPdxFine(&'a OpDPdxFine),
    DPdyFine(&'a OpDPdyFine),
    FwidthFine(&'a OpFwidthFine),
    DPdxCoarse(&'a OpDPdxCoarse),
    DPdyCoarse(&'a OpDPdyCoarse),
    FwidthCoarse(&'a OpFwidthCoarse),
    Phi(&'a OpPhi),
    LoopMerge(&'a OpLoopMerge),
    SelectionMerge(&'a OpSelectionMerge),
    Label(&'a OpLabel),
    Branch(&'a OpBranch),
    BranchConditional(&'a OpBranchConditional),
    Switch(&'a OpSwitch),
    Kill(&'a OpKill),
    Return(&'a OpReturn),
    ReturnValue(&'a OpReturnValue),
    Unreachable(&'a OpUnreachable),
    LifetimeStart(&'a OpLifetimeStart),
    LifetimeStop(&'a OpLifetimeStop),
    AtomicLoad(&'a OpAtomicLoad),
    AtomicStore(&'a OpAtomicStore),
    AtomicExchange(&'a OpAtomicExchange),
    AtomicCompareExchange(&'a OpAtomicCompareExchange),
    AtomicCompareExchangeWeak(&'a OpAtomicCompareExchangeWeak),
    AtomicIIncrement(&'a OpAtomicIIncrement),
    AtomicIDecrement(&'a OpAtomicIDecrement),
    AtomicIAdd(&'a OpAtomicIAdd),
    AtomicISub(&'a OpAtomicISub),
    AtomicUMin(&'a OpAtomicUMin),
    AtomicUMax(&'a OpAtomicUMax),
    AtomicAnd(&'a OpAtomicAnd),
    AtomicOr(&'a OpAtomicOr),
    AtomicXor(&'a OpAtomicXor),
    AtomicIMin(&'a OpAtomicIMin),
    AtomicIMax(&'a OpAtomicIMax),
    EmitVertex(&'a OpEmitVertex),
    EndPrimitive(&'a OpEndPrimitive),
    EmitStreamVertex(&'a OpEmitStreamVertex),
    EndStreamPrimitive(&'a OpEndStreamPrimitive),
    ControlBarrier(&'a OpControlBarrier),
    MemoryBarrier(&'a OpMemoryBarrier),
    GroupAsyncCopy(&'a OpGroupAsyncCopy),
    GroupWaitEvents(&'a OpGroupWaitEvents),
    GroupAll(&'a OpGroupAll),
    GroupAny(&'a OpGroupAny),
    GroupBroadcast(&'a OpGroupBroadcast),
    GroupIAdd(&'a OpGroupIAdd),
    GroupFAdd(&'a OpGroupFAdd),
    GroupFMin(&'a OpGroupFMin),
    GroupUMin(&'a OpGroupUMin),
    GroupSMin(&'a OpGroupSMin),
    GroupFMax(&'a OpGroupFMax),
    GroupUMax(&'a OpGroupUMax),
    GroupSMax(&'a OpGroupSMax),
    EnqueueMarker(&'a OpEnqueueMarker),
    EnqueueKernel(&'a OpEnqueueKernel),
    GetKernelNDrangeSubGroupCount(&'a OpGetKernelNDrangeSubGroupCount),
    GetKernelNDrangeMaxSubGroupSize(&'a OpGetKernelNDrangeMaxSubGroupSize),
    GetKernelWorkGroupSize(&'a OpGetKernelWorkGroupSize),
    GetKernelPreferredWorkGroupSizeMultiple(&'a OpGetKernelPreferredWorkGroupSizeMultiple),
    RetainEvent(&'a OpRetainEvent),
    ReleaseEvent(&'a OpReleaseEvent),
    CreateUserEvent(&'a OpCreateUserEvent),
    IsValidEvent(&'a OpIsValidEvent),
    SetUserEventStatus(&'a OpSetUserEventStatus),
    CaptureEventProfilingInfo(&'a OpCaptureEventProfilingInfo),
    GetDefaultQueue(&'a OpGetDefaultQueue),
    BuildNDRange(&'a OpBuildNDRange),
    ReadPipe(&'a OpReadPipe),
    WritePipe(&'a OpWritePipe),
    ReservedReadPipe(&'a OpReservedReadPipe),
    ReservedWritePipe(&'a OpReservedWritePipe),
    ReserveReadPipePackets(&'a OpReserveReadPipePackets),
    ReserveWritePipePackets(&'a OpReserveWritePipePackets),
    CommitReadPipe(&'a OpCommitReadPipe),
    CommitWritePipe(&'a OpCommitWritePipe),
    IsValidReserveId(&'a OpIsValidReserveId),
    GetNumPipePackets(&'a OpGetNumPipePackets),
    GetMaxPipePackets(&'a OpGetMaxPipePackets),
    GroupReserveReadPipePackets(&'a OpGroupReserveReadPipePackets),
    GroupReserveWritePipePackets(&'a OpGroupReserveWritePipePackets),
    GroupCommitReadPipe(&'a OpGroupCommitReadPipe),
    GroupCommitWritePipe(&'a OpGroupCommitWritePipe)
}

impl<'a> Instruction<'a> {
    pub unsafe fn from_opcode_and_ptr(opcode: Op, ptr: *const u32) -> Self {
        match opcode {
            Op::Nop => Instruction::Nop,
            Op::Undef => Instruction::Undef(&*(ptr as *const _)),
            Op::Source => Instruction::Source(&*(ptr as *const _)),
            Op::SourceExtension => Instruction::SourceExtension(&*(ptr as *const _)),
            Op::Name => Instruction::Name(&*(ptr as *const _)),
            Op::MemberName => Instruction::MemberName(&*(ptr as *const _)),
            Op::String => Instruction::String(&*(ptr as *const _)),
            Op::Line => Instruction::Line(&*(ptr as *const _)),
            Op::Capability => Instruction::Capability(&*(ptr as *const _)),
            Op::DecorationGroup => Instruction::DecorationGroup(&*(ptr as *const _)),
            Op::Decorate => Instruction::Decorate(&*(ptr as *const _)),
            Op::MemberDecorate => Instruction::MemberDecorate(&*(ptr as *const _)),
            Op::GroupDecorate => Instruction::GroupDecorate(&*(ptr as *const _)),
            Op::GroupMemberDecorate => Instruction::GroupMemberDecorate(&*(ptr as *const _)),
            Op::Extension => Instruction::Extension(&*(ptr as *const _)),
            Op::ExtInstImport => Instruction::ExtInstImport(&*(ptr as *const _)),
            Op::ExtInst => Instruction::ExtInst(&*(ptr as *const _)),
            Op::MemoryModel => Instruction::MemoryModel(&*(ptr as *const _)),
            Op::EntryPoint => Instruction::EntryPoint(&*(ptr as *const _)),
            Op::ExecutionMode => Instruction::ExecutionMode(&*(ptr as *const _)),
            Op::TypeVoid => Instruction::TypeVoid(&*(ptr as *const _)),
            Op::TypeBool => Instruction::TypeBool(&*(ptr as *const _)),
            Op::TypeInt => Instruction::TypeInt(&*(ptr as *const _)),
            Op::TypeFloat => Instruction::TypeFloat(&*(ptr as *const _)),
            Op::TypeVector => Instruction::TypeVector(&*(ptr as *const _)),
            Op::TypeMatrix => Instruction::TypeMatrix(&*(ptr as *const _)),
            Op::TypeSampler => Instruction::TypeSampler(&*(ptr as *const _)),
            Op::TypeArray => Instruction::TypeArray(&*(ptr as *const _)),
            Op::TypeRuntimeArray => Instruction::TypeRuntimeArray(&*(ptr as *const _)),
            Op::TypeStruct => Instruction::TypeStruct(&*(ptr as *const _)),
            Op::TypeOpaque => Instruction::TypeOpaque(&*(ptr as *const _)),
            Op::TypePointer => Instruction::TypePointer(&*(ptr as *const _)),
            Op::TypeImage => Instruction::TypeImage(&*(ptr as *const _)),
            Op::TypeSampledImage => Instruction::TypeSampledImage(&*(ptr as *const _)),
            Op::TypeFunction => Instruction::TypeFunction(&*(ptr as *const _)),
            Op::TypeEvent => Instruction::TypeEvent(&*(ptr as *const _)),
            Op::TypeDeviceEvent => Instruction::TypeDeviceEvent(&*(ptr as *const _)),
            Op::TypeReserveId => Instruction::TypeReserveId(&*(ptr as *const _)),
            Op::TypeQueue => Instruction::TypeQueue(&*(ptr as *const _)),
            Op::TypePipe => Instruction::TypePipe(&*(ptr as *const _)),
            Op::ConstantTrue => Instruction::ConstantTrue(&*(ptr as *const _)),
            Op::ConstantFalse => Instruction::ConstantFalse(&*(ptr as *const _)),
            Op::Constant => Instruction::Constant(&*(ptr as *const _)),
            Op::ConstantComposite => Instruction::ConstantComposite(&*(ptr as *const _)),
            Op::ConstantSampler => Instruction::ConstantSampler(&*(ptr as *const _)),
            Op::ConstantNull => Instruction::ConstantNull(&*(ptr as *const _)),
            Op::SpecConstantTrue => Instruction::SpecConstantTrue(&*(ptr as *const _)),
            Op::SpecConstantFalse => Instruction::SpecConstantFalse(&*(ptr as *const _)),
            Op::SpecConstant => Instruction::SpecConstant(&*(ptr as *const _)),
            Op::SpecConstantComposite => Instruction::SpecConstantComposite(&*(ptr as *const _)),
            Op::SpecConstantOp => Instruction::SpecConstantOp(&*(ptr as *const _)),
            Op::Variable => Instruction::Variable(&*(ptr as *const _)),
            Op::Load => Instruction::Load(&*(ptr as *const _)),
            Op::Store => Instruction::Store(&*(ptr as *const _)),
            Op::CopyMemory => Instruction::CopyMemory(&*(ptr as *const _)),
            Op::CopyMemorySized => Instruction::CopyMemorySized(&*(ptr as *const _)),
            Op::AccessChain => Instruction::AccessChain(&*(ptr as *const _)),
            Op::PtrAccessChain => Instruction::PtrAccessChain(&*(ptr as *const _)),
            Op::InBoundsAccessChain => Instruction::InBoundsAccessChain(&*(ptr as *const _)),
            Op::ArrayLength => Instruction::ArrayLength(&*(ptr as *const _)),
            Op::ImageTexelPointer => Instruction::ImageTexelPointer(&*(ptr as *const _)),
            Op::GenericPtrMemSemantics => Instruction::GenericPtrMemSemantics(&*(ptr as *const _)),
            Op::Function => Instruction::Function(&*(ptr as *const _)),
            Op::FunctionParameter => Instruction::FunctionParameter(&*(ptr as *const _)),
            Op::FunctionEnd => Instruction::FunctionEnd(&*(ptr as *const _)),
            Op::FunctionCall => Instruction::FunctionCall(&*(ptr as *const _)),
            Op::SampledImage => Instruction::SampledImage(&*(ptr as *const _)),
            Op::ImageSampleImplicitLod => Instruction::ImageSampleImplicitLod(&*(ptr as *const _)),
            Op::ImageSampleExplicitLod => Instruction::ImageSampleExplicitLod(&*(ptr as *const _)),
            Op::ImageSampleDrefImplicitLod => Instruction::ImageSampleDrefImplicitLod(&*(ptr as *const _)),
            Op::ImageSampleDrefExplicitLod => Instruction::ImageSampleDrefExplicitLod(&*(ptr as *const _)),
            Op::ImageSampleProjImplicitLod => Instruction::ImageSampleProjImplicitLod(&*(ptr as *const _)),
            Op::ImageSampleProjExplicitLod => Instruction::ImageSampleProjExplicitLod(&*(ptr as *const _)),
            Op::ImageSampleProjDrefImplicitLod => Instruction::ImageSampleProjDrefImplicitLod(&*(ptr as *const _)),
            Op::ImageSampleProjDrefExplicitLod => Instruction::ImageSampleProjDrefExplicitLod(&*(ptr as *const _)),
            Op::ImageFetch => Instruction::ImageFetch(&*(ptr as *const _)),
            Op::ImageGather => Instruction::ImageGather(&*(ptr as *const _)),
            Op::ImageDrefGather => Instruction::ImageDrefGather(&*(ptr as *const _)),
            Op::ImageRead => Instruction::ImageRead(&*(ptr as *const _)),
            Op::ImageWrite => Instruction::ImageWrite(&*(ptr as *const _)),
            //Op::ImageQueryDim => Instruction::ImageQueryDim(&*(ptr as *const _)),
            Op::ImageQueryFormat => Instruction::ImageQueryFormat(&*(ptr as *const _)),
            Op::ImageQueryOrder => Instruction::ImageQueryOrder(&*(ptr as *const _)),
            Op::ImageQuerySizeLod => Instruction::ImageQuerySizeLod(&*(ptr as *const _)),
            Op::ImageQuerySize => Instruction::ImageQuerySize(&*(ptr as *const _)),
            Op::ImageQueryLod => Instruction::ImageQueryLod(&*(ptr as *const _)),
            Op::ImageQueryLevels => Instruction::ImageQueryLevels(&*(ptr as *const _)),
            Op::ImageQuerySamples => Instruction::ImageQuerySamples(&*(ptr as *const _)),
            Op::ConvertFToU => Instruction::ConvertFToU(&*(ptr as *const _)),
            Op::ConvertFToS => Instruction::ConvertFToS(&*(ptr as *const _)),
            Op::ConvertSToF => Instruction::ConvertSToF(&*(ptr as *const _)),
            Op::ConvertUToF => Instruction::ConvertUToF(&*(ptr as *const _)),
            Op::UConvert => Instruction::UConvert(&*(ptr as *const _)),
            Op::SConvert => Instruction::SConvert(&*(ptr as *const _)),
            Op::FConvert => Instruction::FConvert(&*(ptr as *const _)),
            Op::QuantizeToF16 => Instruction::QuantizeToF16(&*(ptr as *const _)),
            Op::ConvertPtrToU => Instruction::ConvertPtrToU(&*(ptr as *const _)),
            Op::ConvertUToPtr => Instruction::ConvertUToPtr(&*(ptr as *const _)),
            Op::PtrCastToGeneric => Instruction::PtrCastToGeneric(&*(ptr as *const _)),
            Op::GenericCastToPtr => Instruction::GenericCastToPtr(&*(ptr as *const _)),
            Op::Bitcast => Instruction::Bitcast(&*(ptr as *const _)),
            Op::GenericCastToPtrExplicit => Instruction::GenericCastToPtrExplicit(&*(ptr as *const _)),
            Op::SatConvertSToU => Instruction::SatConvertSToU(&*(ptr as *const _)),
            Op::SatConvertUToS => Instruction::SatConvertUToS(&*(ptr as *const _)),
            Op::VectorExtractDynamic => Instruction::VectorExtractDynamic(&*(ptr as *const _)),
            Op::VectorInsertDynamic => Instruction::VectorInsertDynamic(&*(ptr as *const _)),
            Op::VectorShuffle => Instruction::VectorShuffle(&*(ptr as *const _)),
            Op::CompositeConstruct => Instruction::CompositeConstruct(&*(ptr as *const _)),
            Op::CompositeExtract => Instruction::CompositeExtract(&*(ptr as *const _)),
            Op::CompositeInsert => Instruction::CompositeInsert(&*(ptr as *const _)),
            Op::CopyObject => Instruction::CopyObject(&*(ptr as *const _)),
            Op::Transpose => Instruction::Transpose(&*(ptr as *const _)),
            Op::SNegate => Instruction::SNegate(&*(ptr as *const _)),
            Op::FNegate => Instruction::FNegate(&*(ptr as *const _)),
            Op::Not => Instruction::Not(&*(ptr as *const _)),
            Op::IAdd => Instruction::IAdd(&*(ptr as *const _)),
            Op::FAdd => Instruction::FAdd(&*(ptr as *const _)),
            Op::ISub => Instruction::ISub(&*(ptr as *const _)),
            Op::FSub => Instruction::FSub(&*(ptr as *const _)),
            Op::IMul => Instruction::IMul(&*(ptr as *const _)),
            Op::FMul => Instruction::FMul(&*(ptr as *const _)),
            Op::UDiv => Instruction::UDiv(&*(ptr as *const _)),
            Op::SDiv => Instruction::SDiv(&*(ptr as *const _)),
            Op::FDiv => Instruction::FDiv(&*(ptr as *const _)),
            Op::UMod => Instruction::UMod(&*(ptr as *const _)),
            Op::SRem => Instruction::SRem(&*(ptr as *const _)),
            Op::SMod => Instruction::SMod(&*(ptr as *const _)),
            Op::FRem => Instruction::FRem(&*(ptr as *const _)),
            Op::FMod => Instruction::FMod(&*(ptr as *const _)),
            Op::VectorTimesScalar => Instruction::VectorTimesScalar(&*(ptr as *const _)),
            Op::MatrixTimesScalar => Instruction::MatrixTimesScalar(&*(ptr as *const _)),
            Op::VectorTimesMatrix => Instruction::VectorTimesMatrix(&*(ptr as *const _)),
            Op::MatrixTimesVector => Instruction::MatrixTimesVector(&*(ptr as *const _)),
            Op::MatrixTimesMatrix => Instruction::MatrixTimesMatrix(&*(ptr as *const _)),
            Op::OuterProduct => Instruction::OuterProduct(&*(ptr as *const _)),
            Op::Dot => Instruction::Dot(&*(ptr as *const _)),
            Op::ShiftRightLogical => Instruction::ShiftRightLogical(&*(ptr as *const _)),
            Op::ShiftRightArithmetic => Instruction::ShiftRightArithmetic(&*(ptr as *const _)),
            Op::ShiftLeftLogical => Instruction::ShiftLeftLogical(&*(ptr as *const _)),
            Op::BitwiseOr => Instruction::BitwiseOr(&*(ptr as *const _)),
            Op::BitwiseXor => Instruction::BitwiseXor(&*(ptr as *const _)),
            Op::BitwiseAnd => Instruction::BitwiseAnd(&*(ptr as *const _)),
            Op::Any => Instruction::Any(&*(ptr as *const _)),
            Op::All => Instruction::All(&*(ptr as *const _)),
            Op::IsNan => Instruction::IsNan(&*(ptr as *const _)),
            Op::IsInf => Instruction::IsInf(&*(ptr as *const _)),
            Op::IsFinite => Instruction::IsFinite(&*(ptr as *const _)),
            Op::IsNormal => Instruction::IsNormal(&*(ptr as *const _)),
            Op::SignBitSet => Instruction::SignBitSet(&*(ptr as *const _)),
            Op::LessOrGreater => Instruction::LessOrGreater(&*(ptr as *const _)),
            Op::Ordered => Instruction::Ordered(&*(ptr as *const _)),
            Op::Unordered => Instruction::Unordered(&*(ptr as *const _)),
            Op::LogicalOr => Instruction::LogicalOr(&*(ptr as *const _)),
            Op::LogicalNotEqual => Instruction::LogicalNotEqual(&*(ptr as *const _)),
            Op::LogicalEqual => Instruction::LogicalEqual(&*(ptr as *const _)),
            Op::LogicalNot => Instruction::LogicalNot(&*(ptr as *const _)),
            Op::LogicalAnd => Instruction::LogicalAnd(&*(ptr as *const _)),
            Op::Select => Instruction::Select(&*(ptr as *const _)),
            Op::IEqual => Instruction::IEqual(&*(ptr as *const _)),
            Op::FOrdEqual => Instruction::FOrdEqual(&*(ptr as *const _)),
            Op::FUnordEqual => Instruction::FUnordEqual(&*(ptr as *const _)),
            Op::INotEqual => Instruction::INotEqual(&*(ptr as *const _)),
            Op::FOrdNotEqual => Instruction::FOrdNotEqual(&*(ptr as *const _)),
            Op::FUnordNotEqual => Instruction::FUnordNotEqual(&*(ptr as *const _)),
            Op::ULessThan => Instruction::ULessThan(&*(ptr as *const _)),
            Op::SLessThan => Instruction::SLessThan(&*(ptr as *const _)),
            Op::FOrdLessThan => Instruction::FOrdLessThan(&*(ptr as *const _)),
            Op::FUnordLessThan => Instruction::FUnordLessThan(&*(ptr as *const _)),
            Op::UGreaterThan => Instruction::UGreaterThan(&*(ptr as *const _)),
            Op::SGreaterThan => Instruction::SGreaterThan(&*(ptr as *const _)),
            Op::FOrdGreaterThan => Instruction::FOrdGreaterThan(&*(ptr as *const _)),
            Op::FUnordGreaterThan => Instruction::FUnordGreaterThan(&*(ptr as *const _)),
            Op::ULessThanEqual => Instruction::ULessThanEqual(&*(ptr as *const _)),
            Op::SLessThanEqual => Instruction::SLessThanEqual(&*(ptr as *const _)),
            Op::FOrdLessThanEqual => Instruction::FOrdLessThanEqual(&*(ptr as *const _)),
            Op::FUnordLessThanEqual => Instruction::FUnordLessThanEqual(&*(ptr as *const _)),
            Op::UGreaterThanEqual => Instruction::UGreaterThanEqual(&*(ptr as *const _)),
            Op::SGreaterThanEqual => Instruction::SGreaterThanEqual(&*(ptr as *const _)),
            Op::FOrdGreaterThanEqual => Instruction::FOrdGreaterThanEqual(&*(ptr as *const _)),
            Op::FUnordGreaterThanEqual => Instruction::FUnordGreaterThanEqual(&*(ptr as *const _)),
            Op::DPdx => Instruction::DPdx(&*(ptr as *const _)),
            Op::DPdy => Instruction::DPdy(&*(ptr as *const _)),
            Op::Fwidth => Instruction::Fwidth(&*(ptr as *const _)),
            Op::DPdxFine => Instruction::DPdxFine(&*(ptr as *const _)),
            Op::DPdyFine => Instruction::DPdyFine(&*(ptr as *const _)),
            Op::FwidthFine => Instruction::FwidthFine(&*(ptr as *const _)),
            Op::DPdxCoarse => Instruction::DPdxCoarse(&*(ptr as *const _)),
            Op::DPdyCoarse => Instruction::DPdyCoarse(&*(ptr as *const _)),
            Op::FwidthCoarse => Instruction::FwidthCoarse(&*(ptr as *const _)),
            Op::Phi => Instruction::Phi(&*(ptr as *const _)),
            Op::LoopMerge => Instruction::LoopMerge(&*(ptr as *const _)),
            Op::SelectionMerge => Instruction::SelectionMerge(&*(ptr as *const _)),
            Op::Label => Instruction::Label(&*(ptr as *const _)),
            Op::Branch => Instruction::Branch(&*(ptr as *const _)),
            Op::BranchConditional => Instruction::BranchConditional(&*(ptr as *const _)),
            Op::Switch => Instruction::Switch(&*(ptr as *const _)),
            Op::Kill => Instruction::Kill(&*(ptr as *const _)),
            Op::Return => Instruction::Return(&*(ptr as *const _)),
            Op::ReturnValue => Instruction::ReturnValue(&*(ptr as *const _)),
            Op::Unreachable => Instruction::Unreachable(&*(ptr as *const _)),
            Op::LifetimeStart => Instruction::LifetimeStart(&*(ptr as *const _)),
            Op::LifetimeStop => Instruction::LifetimeStop(&*(ptr as *const _)),
            Op::AtomicLoad => Instruction::AtomicLoad(&*(ptr as *const _)),
            Op::AtomicStore => Instruction::AtomicStore(&*(ptr as *const _)),
            Op::AtomicExchange => Instruction::AtomicExchange(&*(ptr as *const _)),
            Op::AtomicCompareExchange => Instruction::AtomicCompareExchange(&*(ptr as *const _)),
            Op::AtomicCompareExchangeWeak => Instruction::AtomicCompareExchangeWeak(&*(ptr as *const _)),
            Op::AtomicIIncrement => Instruction::AtomicIIncrement(&*(ptr as *const _)),
            Op::AtomicIDecrement => Instruction::AtomicIDecrement(&*(ptr as *const _)),
            Op::AtomicIAdd => Instruction::AtomicIAdd(&*(ptr as *const _)),
            Op::AtomicISub => Instruction::AtomicISub(&*(ptr as *const _)),
            Op::AtomicUMin => Instruction::AtomicUMin(&*(ptr as *const _)),
            Op::AtomicUMax => Instruction::AtomicUMax(&*(ptr as *const _)),
            Op::AtomicAnd => Instruction::AtomicAnd(&*(ptr as *const _)),
            Op::AtomicOr => Instruction::AtomicOr(&*(ptr as *const _)),
            Op::AtomicXor => Instruction::AtomicXor(&*(ptr as *const _)),
            //Op::OpAtomicIMin => Instruction::AtomicIMin(&*(ptr as *const _)),
            //Op::OpAtomicIMax => Instruction::AtomicIMax(&*(ptr as *const _)),
            Op::EmitVertex => Instruction::EmitVertex(&*(ptr as *const _)),
            Op::EndPrimitive => Instruction::EndPrimitive(&*(ptr as *const _)),
            Op::EmitStreamVertex => Instruction::EmitStreamVertex(&*(ptr as *const _)),
            Op::EndStreamPrimitive => Instruction::EndStreamPrimitive(&*(ptr as *const _)),
            Op::ControlBarrier => Instruction::ControlBarrier(&*(ptr as *const _)),
            Op::MemoryBarrier => Instruction::MemoryBarrier(&*(ptr as *const _)),
            Op::GroupAsyncCopy => Instruction::GroupAsyncCopy(&*(ptr as *const _)),
            Op::GroupWaitEvents => Instruction::GroupWaitEvents(&*(ptr as *const _)),
            Op::GroupAll => Instruction::GroupAll(&*(ptr as *const _)),
            Op::GroupAny => Instruction::GroupAny(&*(ptr as *const _)),
            Op::GroupBroadcast => Instruction::GroupBroadcast(&*(ptr as *const _)),
            Op::GroupIAdd => Instruction::GroupIAdd(&*(ptr as *const _)),
            Op::GroupFAdd => Instruction::GroupFAdd(&*(ptr as *const _)),
            Op::GroupFMin => Instruction::GroupFMin(&*(ptr as *const _)),
            Op::GroupUMin => Instruction::GroupUMin(&*(ptr as *const _)),
            Op::GroupSMin => Instruction::GroupSMin(&*(ptr as *const _)),
            Op::GroupFMax => Instruction::GroupFMax(&*(ptr as *const _)),
            Op::GroupUMax => Instruction::GroupUMax(&*(ptr as *const _)),
            Op::GroupSMax => Instruction::GroupSMax(&*(ptr as *const _)),
            Op::EnqueueMarker => Instruction::EnqueueMarker(&*(ptr as *const _)),
            Op::EnqueueKernel => Instruction::EnqueueKernel(&*(ptr as *const _)),
            Op::GetKernelNDrangeSubGroupCount => Instruction::GetKernelNDrangeSubGroupCount(&*(ptr as *const _)),
            Op::GetKernelNDrangeMaxSubGroupSize => Instruction::GetKernelNDrangeMaxSubGroupSize(&*(ptr as *const _)),
            Op::GetKernelWorkGroupSize => Instruction::GetKernelWorkGroupSize(&*(ptr as *const _)),
            Op::GetKernelPreferredWorkGroupSizeMultiple => Instruction::GetKernelPreferredWorkGroupSizeMultiple(&*(ptr as *const _)),
            Op::RetainEvent => Instruction::RetainEvent(&*(ptr as *const _)),
            Op::ReleaseEvent => Instruction::ReleaseEvent(&*(ptr as *const _)),
            Op::CreateUserEvent => Instruction::CreateUserEvent(&*(ptr as *const _)),
            Op::IsValidEvent => Instruction::IsValidEvent(&*(ptr as *const _)),
            Op::SetUserEventStatus => Instruction::SetUserEventStatus(&*(ptr as *const _)),
            Op::CaptureEventProfilingInfo => Instruction::CaptureEventProfilingInfo(&*(ptr as *const _)),
            Op::GetDefaultQueue => Instruction::GetDefaultQueue(&*(ptr as *const _)),
            Op::BuildNDRange => Instruction::BuildNDRange(&*(ptr as *const _)),
            Op::ReadPipe => Instruction::ReadPipe(&*(ptr as *const _)),
            Op::WritePipe => Instruction::WritePipe(&*(ptr as *const _)),
            Op::ReservedReadPipe => Instruction::ReservedReadPipe(&*(ptr as *const _)),
            Op::ReservedWritePipe => Instruction::ReservedWritePipe(&*(ptr as *const _)),
            Op::ReserveReadPipePackets => Instruction::ReserveReadPipePackets(&*(ptr as *const _)),
            Op::ReserveWritePipePackets => Instruction::ReserveWritePipePackets(&*(ptr as *const _)),
            Op::CommitReadPipe => Instruction::CommitReadPipe(&*(ptr as *const _)),
            Op::CommitWritePipe => Instruction::CommitWritePipe(&*(ptr as *const _)),
            Op::IsValidReserveId => Instruction::IsValidReserveId(&*(ptr as *const _)),
            Op::GetNumPipePackets => Instruction::GetNumPipePackets(&*(ptr as *const _)),
            Op::GetMaxPipePackets => Instruction::GetMaxPipePackets(&*(ptr as *const _)),
            Op::GroupReserveReadPipePackets => Instruction::GroupReserveReadPipePackets(&*(ptr as *const _)),
            Op::GroupReserveWritePipePackets => Instruction::GroupReserveWritePipePackets(&*(ptr as *const _)),
            Op::GroupCommitReadPipe => Instruction::GroupCommitReadPipe(&*(ptr as *const _)),
            Op::GroupCommitWritePipe => Instruction::GroupCommitWritePipe(&*(ptr as *const _)),
            Op::IAddCarry => unimplemented!(),
            Op::ISubBorrow => unimplemented!(),
            Op::SourceContinued | Op::NoLine => unimplemented!(),
            Op::TypeForwardPointer => unimplemented!(),
            Op::InBoundsPtrAccessChain => unimplemented!(),
            Op::Image => unimplemented!(),
            Op::UMulExtended => unimplemented!(),
            Op::SMulExtended => unimplemented!(),
            Op::AtomicFlagTestAndSet | Op::AtomicFlagClear => unimplemented!(),
            Op::ImageSparseSampleImplicitLod | Op::ImageSparseSampleExplicitLod | Op::ImageSparseSampleDrefImplicitLod | Op::ImageSparseSampleDrefExplicitLod | Op::ImageSparseSampleProjImplicitLod | Op::ImageSparseSampleProjExplicitLod | Op::ImageSparseSampleProjDrefImplicitLod | Op::ImageSparseSampleProjDrefExplicitLod => unimplemented!(),
            Op::ImageSparseFetch | Op::ImageSparseGather | Op::ImageSparseDrefGather | Op::ImageSparseTexelsResident => unimplemented!(),
            Op::BitFieldInsert | Op::BitFieldSExtract | Op::BitFieldUExtract | Op::BitReverse | Op::BitCount | Op::AtomicSMin | Op::AtomicSMax => unimplemented!(),
            Op::Count => unreachable!()
        }
    }
}

///---------------------------------
///3.27.1 Miscellaneous Instructions
///---------------------------------

// Use is invalid.
instruction! { OpNop, 1, 1 }

// Make an intermediate object with no initialization.
instruction! { OpUndef, 3, 3,
    // The type of object to make.
    result_type: Id,
    result_id: Id
}

//-------------------------------------
//3.27.2 Debug (Removable) Instructions
//-------------------------------------

instruction! { OpSource, 2, 2,
    // Document what source language this module was translated from.
    // This has no semantic impact and can safely be removed from a module.
    source_language: SourceLanguage,
    // The version of the source language.
    version: LiteralNumber
}

// Document an extension to the source language.
// This has no semantic impact and can safely be removed from a module.
instruction! { OpSourceExtension, 1, 65535,
    // A string describing a source-language extension.
    // Its form is dependent on the how the source language describes extensions.
    extension: LiteralString
}

// Name a Result <id>.
// This has no semantic impact and can safely be removed from a module
instruction! { OpName, 2, 65535,
    // The Result <id> to name. It can be the Result <id> of any instruction: other,
    // a variable, function, type, intermediate result, etc.
    target: Id,
    // The string to name <id> with.
    name: LiteralString
}

// Name a member of a structure type.
// This has no semantic impact and can safely be removed from a module.
instruction! { OpMemberName, 3, 65535,
    // The <id> from an OpTypeStruct instruction.
    ty: Id,
    // The number of the member to name in the structure.
    // The first member is member 0, the next is member 1, ...
    member: LiteralNumber,
    // The string to name the member with.
    name: LiteralString
}

instruction! { OpString, 2, 65535,
    // Name a string for use with other debug instructions (see OpLine).
    // This has no semantic impact and can safely be removed from a module
    result_id: Id,
    string: LiteralString             // The literal string being assigned a Result <id>.
                                            // It has no result type and no storage
}

// Add source-level location information.
// This has no semantic impact and can safely be removed from a module.
instruction! { OpLine, 5, 5,
    file: Id,               // The <id> from an OpString instruction and is the
                                            // source-level file name.
    line: LiteralNumber,               // The source-level line number.
    column: LiteralNumber             // The source-level column number.
}


//------------------------------
//3.27.3 Annotation Instructions
//------------------------------



// A collector of decorations from OpDecorate instructions.
// All such instructions must precede this instruction.
// Subsequent OpGroupDecorate and OpGroupMemberDecorate
// instructions can consume the Result <id> to apply multiple
// decorations to multiple target <id>s. Those are the only
// instructions allowed to consume the Result <id>.
instruction! { OpDecorationGroup, 2, 2,
    result_id: Id
}

// Add a decoration to another <id>.
instruction! { OpDecorate, 3, 65535,
    target: Id,             // The <id> to decorate. It can potentially be any <id>
                                            // that is a forward reference. A set of decorations can be
                                            // grouped together by having multiple OpDecorate instructions
                                            // target the same OpDecorationGroup instruction.
    decoration: Decoration,
    values: UnsizedArray<LiteralNumber>
}

// Add a decoration to a member of a structure type.
instruction! { OpMemberDecorate, 4, 65535,
    structure_type: Id,      // The <id> of a type from OpTypeStruct.
    member: LiteralNumber,             // The number of the member to decorate in the structure.
                                            // The first member is member 0, the next is member 1, ...
    decoration: Decoration,
    values: UnsizedArray<LiteralNumber>
}

// Add a group of decorations to another <id>.
instruction! { OpGroupDecorate, 2, 65535,
    decorationGroup: Id,    // The <id> of an OpDecorationGroup instruction.
    targets: UnsizedArray<Id>        // The target <id>s to decorate with the groups of decorations
}

// Add a decoration to a member of a structure type.
instruction! { OpGroupMemberDecorate, 2, 65535,
    decorationGroup: Id,    // The <id> of an OpDecorationGroup instruction.
    targets: UnsizedArray<Id>        // The target <id>s to decorate with the groups of decorations.
}



//-----------------------------
//3.27.4 Extension Instructions
//-----------------------------

// Declare use of an extension to SPIR-V. This allows
// validation of additional instructions, tokens, semantics, etc.
instruction! { OpExtension, 1, 65535,
    name: LiteralString               // the extension's name string
}

// Import an extended set of instructions. It can be later referenced by the Result <id>.
// See Extended Instruction Sets for more information.
instruction! { OpExtInstImport, 2, 65535,
    result_id: Id,
    name: LiteralString               // The extended instruction-set's name string.
}

// Execute an instruction in an imported set of extended instructions.
instruction! { OpExtInst, 5, 65535,
    result_type: Id,
    result_id: Id,
    set: Id,                // The result of an OpExtInstImport instruction.
    instruction: LiteralNumber,        // The enumerant of the instruction to execute within the extended instruction Set.
    operands: UnsizedArray<Id>        // The operands to the extended instruction.
}



//--------------------------------
//3.27.5 Mode-Setting Instructions
//--------------------------------

instruction! { OpMemoryModel, 3, 3,
// Set addressing model and memory model for the entire module
    addressing_model: AddressingModel,    // Selects the module's addressing model, see Addressing Model.
    memory_model: MemoryModel        // Selects the module's memory model, see Memory Model.
}

instruction! { OpEntryPoint, 3, 3,
// Declare an entry point and its execution model.
    execution_model: ExecutionModel,     // The execution model for the entry point and its static call tree.
                                            // See Execution Model.
    result: Id            // The Result <id> of an OpFunction instruction.
}

// Declare an execution mode for an entry point.
instruction! { OpExecutionMode, 3, 65535,
    entry_point: Id,         // Must be the Entry Point <id> operand of an OpEntryPoint instruction.
    mode: ExecutionMode,               // The execution mode. See Execution Mode.
    literals: UnsizedArray<LiteralNumber>
}

// Declare a capability used by this module.
// This has no semantic impact and can safely be removed from a module.
instruction! { OpCapability, 2, 2,
    // Capability is the capability declared by this instruction.
    // There are no restrictions on the order in which capabilities are declared.
    capability: Capability
}

//------------------------------------
//3.27.6 Type-Declaration Instructions
//------------------------------------

// Declare the void type.
instruction_base! { OpTypeVoid, 2, 2 }

// Declare the Boolean type. Values of this type can only be either true or false.
// There is no physical size or bit pattern defined for these values.
// If they are stored(in conjuction with OpVariable),
// they can only be used with logical addressing operations, not physical,
// and only with non-externally visible shader storage classes :
// WorkgroupLocal, WorkgroupGlobal, PrivateGlobal, and Function.
instruction_base! { OpTypeBool, 2, 2 }

// Declare a new integer type.
instruction_base! { OpTypeInt, 4, 4,
    width: LiteralNumber,              // Specifies how many bits wide the type is.
                                            // The bit pattern of a signed integer value is two's complement.
    signedness: LiteralNumber         // Specifies whether there are signed semantics to preserve or validate.
                                            //     0 indicates unsigned, or no signedness semantics
                                            //     1 indicates signed semantics.
                                            // In all cases, the type of operation of an instruction comes from
                                            // the instruction's opcode, not the signedness of the operands
}

// Declare a new floating-point type.
instruction_base! { OpTypeFloat, 3, 3,
    width: LiteralNumber              // Specifies how many bits wide the type is.
                                            // The bit pattern of a floating-point value is as described by the IEEE 754 standard.
}

// Declare a new vector type.
instruction_base! { OpTypeVector, 4, 4,
    component_type: Id,      // The type of each component in the resulting type.
    component_count: LiteralNumber     // The number of compononents in the resulting type.
                                            // It must be at least 2.
}

// Declare a new matrix type.
// Capability: Matrix
instruction_base! { OpTypeMatrix, 4, 4,
    column_type: Id,         // The type of each column in the matrix. It must be vector type.
    column_count: LiteralNumber        // The number of columns in the new matrix type.
                                            // It must be at least 2.
}


// Declare a new sampler type. Consumed, for example,
// by OpImageSample. This type is opaque: values of
// this type have no defined physical size or bit pattern.
instruction_base! { OpTypeSampler, 8, 9,
    sampled_type: Id,        // A scalar type, of the type of the components resulting
                                            // from sampling or loading through this sampler.

    Dim: Dim,     // Is the texture Dim.

    content: LiteralNumber,            // Must be one of the following indicated values:
                                            //     0 indicates a texture, no filter(no sampling state)
                                            //     1 indicates an image
                                            //     2 indicates both a texture and filter(sampling state),
                                            //       see OpTypeFilte

    arrayed: LiteralNumber,            // Must be one of the following indicated values:
                                            //     0 indicates non- arrayed content
                                            //     1 indicates arrayed content

    compare: LiteralNumber,            // Must be one of the following indicated values:
                                            //     0 indicates depth comparisons are not done
                                            //     1 indicates depth comparison are done

    multisampled: LiteralNumber,       // Must be one of the following indicated values:
                                            //     0 indicates single- sampled content
                                            //     1 indicates multisampled content

    qualifier: Id          //(optional) An image access qualifier. See Access Qualifier.

}

// Declare a new array type: a dynamically-indexable
// ordered aggregate of elements all having the same type.
instruction_base! { OpTypeArray, 4, 4,
    element_type: Id,        // The type of each element in the array.
    length: Id             // The number of elements in the array. It must be at least 1.
                                            // Length must come from a constant instruction of an
                                            // Integer-type scalar whose value is at least 1.
}

instruction_base! { OpTypeRuntimeArray, 3, 3,
// Declare a new run-time array type.
// Its length is not known at compile time.
// Objects of this type can only be created with OpVariable using the Uniform
// Storage Class.
// Capability: Shader
    element_type: Id       // The type of each element in the array. See OpArrayLength for
                                            // getting the Length of an array of this type.
}

instruction_base! { OpTypeStruct, 2, 65535,
// Declare a new structure type: an aggregate of heteregeneous members
    member_types: UnsizedArray<Id>      // The type of member N of the structure.
                                            // The first member is member 0, the next is member 1, ...
}

instruction_base! { OpTypeOpaque, 2, 65535,
// Declare a named structure type with no body specified.
// Capability: Kernel
    opaque_type: LiteralString        // The name of the opaque type.
}

instruction_base! { OpTypePointer, 4, 4,
// Declare a new pointer type.
    storage_class: StorageClass,       // The Storage Class of the memory holding the object pointed to.
    ty: Id              // The type of the object pointed to.
}

// Declare a new image type.
instruction_base! { OpTypeImage, 9, 10,
    sampled_type: Id,
    dimension: Dim,
    depth: LiteralNumber,
    arrayed: LiteralNumber,
    multisample: LiteralNumber,
    sampled: LiteralNumber,
    image_format: ImageFormat,
    access_qualifier: AccessQualifier
}

// Declare a new sampled image type.
instruction_base! { OpTypeSampledImage, 3, 3,
    image_type: Id
}

// Declare a new function type. OpFunction and OpFunctionDecl,
// will use this to declare the return type and parameter types
// of a function.
instruction_base! { OpTypeFunction, 3, 65535,
    return_type: Id,         // The type of the return value of functions of this type.
                                            // If the function has no return value, Return Type should
                                            // be from OpTypeVoid.
    parameter_type: UnsizedArray<Id>  // The type <id> of the type of parameter N.
}

// Declare an OpenCL event object.
// Capability: Kernel
instruction_base! { OpTypeEvent, 2, 2 }

// Declare an OpenCL device-side event object.
// Capability: Kernel
instruction_base! { OpTypeDeviceEvent, 2, 2 }

// Declare an OpenCL reservation id object.
// Capability: Kernel
instruction_base! { OpTypeReserveId, 2, 2 }

// Declare an OpenCL queue object.
// Capability: Kernel
instruction_base! { OpTypeQueue, 2, 2 }

// Declare an OpenCL pipe object type.
// Capability: Kernel
instruction_base! { OpTypePipe, 4, 4,
    // The data type of the pipe.
    ty: Id,
    // The pipe access qualifier.
    qualifier: AccessQualifier
}



//-------------------------------------
//3.27.7 Constant-Creation Instructions
//-------------------------------------

// Declare a true Boolean-type scalar constant.
instruction! { OpConstantTrue, 3, 3,
    result_type: Id,         // Must be the scalar Boolean type
    result: Id
}

instruction! { OpConstantFalse, 3, 3,
// Declare a false Boolean-type scalar constant.
    result_type: Id,         // Must be the scalar Boolean type
    result: Id
}

instruction! { OpConstant, 3, 65535,
// Declare a new Integer-type or Floating-point-type scalar constant.
    result_type: Id,         // Must be a scalar Integer type or Floating-point type.
    result: Id,
    value: LiteralNumber           // The bit pattern for the constant.
                                            // Types 32 bits wide or smaller take one word.
                                            // Larger types take multiple words,
                                            // with low-order words appearing first.
}

// Declare a new composite constant.
instruction! { OpConstantComposite, 3, 65535,
    // Must be a composite type, whose top-level members /
    // elements / components / columns have the same type
    // as the types of the operands.
    result_type: Id,
    result: Id,
    // Constituents will become members of a structure,
    // or elements of an array, or components of a vector,
    // or columns of a matrix. There must be exactly one
    // Constituent for each top-level member / element /
    // component / column of the result. The Constituents
    // Must appear in the order needed by the definition of
    // the type of the result. The Constituents must be
    // the <id> of other constant declarations.
    constituents: UnsizedArray<Id>
}

// Declare a new sampler constant.
// Capability: Kernel
instruction! { OpConstantSampler, 6, 6,
    result_type: Id,
    result: Id,
    mode: LiteralNumber,               // The addressing Mode. See Sampler Addressing Mode.
    param: LiteralNumber,              // One of:
                                            //     0 Nonparametric
                                            //     1 Parametric
    filter: LiteralNumber            // The filter mode. See Sampler Filter Mode.
}

// Declare a new null constant.
// Capability: Addr
instruction! { OpConstantNull, 3, 3,
    result_type: Id,
    result: Id
}

instruction! { OpSpecConstantTrue, 3, 3,
// Declare a Boolean-type scalar specialization
// constant with a default value of true.
// This instruction can be specialized to become
// either an OpConstantTrue or OpConstantFalse
// instruction.
// See Specialization.
// Capability: Shader
    result_type: Id,         // Must be the scalar Boolean type.
    result: Id
}

instruction! { OpSpecConstantFalse, 3, 3,
// Declare a Boolean-type scalar specialization
// constant with a default value of false.
// This instruction can be specialized to become
// either an OpConstantTrue or OpConstantFalse
// instruction.
// See Specialization.
// Capability: Shader
    result_type: Id,         // Must be the scalar Boolean type.
    result: Id
}

instruction! { OpSpecConstant, 3, 65535,
// Declare a new Integer-type or Floating-point-type
// scalar specialization constant.
// This instruction can be specialized to become
// an OpConstant instruction.
// See Specialization.
// Capability: Shader
    result_type: Id,
    result: Id,
    value: [LiteralNumber; 1]           // The bit pattern for the default value
                                            // of the constant. Types 32 bits wide or smaller
                                            // take one word. Larger types take multiple words,
                                            // with low-order words appearing first.
}

// Declare a new composite specialization constant.
// This instruction will be specialized to an
// OpConstantComposite instruction.
// See Specialization.
// Capability: Shader
instruction! { OpSpecConstantComposite, 3, 65535,
    result_type: Id,         // Must be a composite type, whose top-level members /
                                            // elements / components / columns have the same type
                                            // as the types of the operands.
    result: Id,
    constituents: UnsizedArray<Id>    // Constituents will become members of a structure,
                                            // or elements of an array, or components
                                            // of a vector, or columns of a matrix.
                                            // There must be exactly one Constituent for each
                                            // top-level member / element / component / column
                                            // of the result.
                                            // The Constituents must appear in the order needed
                                            // by the definition of the type of the result.
                                            // The Constituents must be the <id> of other
                                            // specialization constant or constant declarations.
}

// Declare a new specialization constant that results from doing an operation.
instruction! { OpSpecConstantOp, 3, 65535,
    result_type: Id,         // Result Type must be the type required by the Result Type of Opcode.
    result: Id,
    opcode: Id,
    operands: UnsizedArray<Id>
}


//--------------------------
//3.27.8 Memory Instructions
//--------------------------



instruction! { OpVariable, 4, 5,
// Allocate an object in memory, resulting in a
// pointer to it, which can be used with OpLoad and OpStore
    result_type: Id,         // A type from OpTypePointer, where the type pointed
                                            // to is the type of object in memory.
    result: Id,
    storage_class: StorageClass,       // The kind of memory holding the object.
    initializer: Id        //(optional) If Initializer is present, it will be
                                            // the initial value of the variable's memory content.
                                            // Initializer must be an <id> from a constant
                                            // instruction. Initializer must have the same type
                                            // as the type pointed to by Result Type.
}

instruction! { OpLoad, 4, 65535,
// Load through a pointer.
    result_type: Id,
    result: Id,
    pointer: Id,            // The pointer to load through. It must have a type of
                                            // OpTypePointer whose operand is the same as Result Type.
    memory_access_shift: [MemoryAccessShift; 1]   // Must be a Memory Access literal. See Memory Access for more detail.
}

instruction! { OpStore, 3, 65535,
// Store through a pointer.
    pointer: Id,            // The pointer to store through. It must have a type of
                                            // OpTypePointer whose operand is the same as the type of Object.
    object: Id,             // the object to store.
    memory_access_shift: [MemoryAccessShift; 1]   // Must be a Memory Access literal. See Memory Access for more detail.
}

instruction! { OpCopyMemory, 3, 65535,
// Copy from the memory pointed to by Source to
// the memory pointed to by Target. Both operands
// Must be non-void pointers of the same type.
// Matching storage class is not required.
// The amount of memory copied is the size of
// the type pointed to.
    target: Id,
    source: Id,
    memory_access_shift: [MemoryAccessShift; 1]   // Must be a Memory Access literal. See Memory Access for more detail.
}

instruction! { OpCopyMemorySized, 4, 65535,
// Copy from the memory pointed to by Source to
// the memory pointed to by Target.
    target: Id,
    source: Id,
    size: Id,
    memory_access_shift: [MemoryAccessShift; 1]   // Must be a Memory Access literal. See Memory Access for more detail.
}

// Create a pointer into a composite object
// that can be used with OpLoad and OpStore.
// The storage class of the pointer created
// will be the same as the storage class of
// the base operand.
instruction! { OpAccessChain, 4, 65535,
    result_type: Id,
    result: Id,
    // Must be a pointer type, pointing to the base of the object.
    base: Id,
    // Indexes walk the type hierarchy to the desired depth,
    // potentially down to scalar granularity.
    // The type of the pointer created will be to the type reached
    // by walking the type hierarchy down to the last provided index.
    indexes: UnsizedArray<Id>
}

// Has the same semantics as OpAccessChain, with the addition of the Element operand.
instruction! { OpPtrAccessChain, 5, 65535,
    result_type: Id,
    result: Id,
    // Must be a pointer type, pointing to the base of the object.
    base: Id,
    // Element is used to do the initial dereference of Base: Base is treated as the address of the first
    // element of an array, and the Element element’s address is computed to be the base for the Indexes, as
    // per OpAccessChain. The type of Base after being dereferenced with Element is still the same as the
    // original type of Base.
    element: Id,
    // Indexes walk the type hierarchy to the desired depth,
    // potentially down to scalar granularity.
    // The type of the pointer created will be to the type reached
    // by walking the type hierarchy down to the last provided index.
    indexes: UnsizedArray<Id>
}

instruction! { OpInBoundsAccessChain, 4, 65535,
// Has the same semantics as OpAccessChain,
// with the addition that the resulting pointer
// is known to point within the base object.
    result_type: Id,
    result: Id,
    base: Id,
    indices: UnsizedArray<Id>
}

instruction! { OpArrayLength, 5, 5,
// Result is the array length of a run-time array.
// Capability: Shader
    result_type: Id,
    result: Id,
    structure: Id,          // Must be an object of type OpTypeStruct that contains a
                                            // member that is a run-time array.
    array_member: LiteralNumber       // A member number of Structure that must have a type from
                                            // OpTypeRuntimeArray.
}

instruction! { OpImageTexelPointer, 6, 6,
// Form a pointer to a texel of an image.
// Use of such a pointer is limited to atomic operations.
// TBD. This requires an Image storage class to be added.
    result_type: Id,
    result: Id,
    image: Id,              // A pointer to a variable of type of OpTypeSampler.
    coordinate: Id,         // Specify which texel and sample within the
                                            // image to form an address of.
    sample: Id            // specify which texel and sample within the
                                            // Image to form an address of.
}

instruction! { OpGenericPtrMemSemantics, 4, 4,
// Returns a valid Memory Semantics value for ptr.
// ptr must point to Generic.
// Capability: Kernel
    result_type: Id,         // Must be a 32-bits wide OpTypeInt value.
    result: Id,
    ptr: Id
}



//----------------------------
//3.27.9 Function Instructions
//----------------------------



instruction! { OpFunction, 5, 5,
// Add a function. This instruction must be immediately
// followed by one OpFunctionParameter instruction per
// each formal parameter of this function.
// This function's body or declaration will terminate
// with the next OpFunctionEnd instruction.
    result_type: Id,         // Must be the same as the Return Type declared in Function Type.
    result: Id,
    function_control: FunctionControlShift,
    function_type: Id       // The result of an OpTypeFunction, which declares the types of
                                            // the return value and parameters of the function.
}

instruction! { OpFunctionParameter, 3, 3,
// Declare the <id> for a formal parameter belonging
// to the current function.
// This instruction must immediately follow an
// OpFunction or OpFunctionParameter instruction.
// The order of contiguous OpFunctionParameter
// instructions is the same order arguments will
// be listed in an OpFunctionCall instruction to this
// function. It is also the same order in which
// Parameter Type operands are listed in the
// OpTypeFunction of the Function Type operand
// for this function's OpFunction instruction.
    result_type: Id,         // for all the OpFunctionParameter instructions
                                            // for a function must be the same as, in order,
                                            // the Parameter Type operands listed in the
                                            // OpTypeFunction of the Function Type operand
                                            // for this function's OpFunction instruction.
    result: Id
}

// Last instruction of a function.
instruction! { OpFunctionEnd, 4, 4 }

instruction! { OpFunctionCall, 4, 65535,
// Call a function
// Note: A forward call is possible because
// there is no missing type information:
// Result Type must match the Return Type of
// the function, and the calling argument
// types must match the formal parameter types.
    result_type: Id,         // The type of the return value of the function.
    result: Id,
    function: Id,           // The <id> of an OpFunction instruction.
                                            // This could be a forward reference.
    arguments: UnsizedArray<Id>      // The <id>s of the object to copy to parameter N of Function
}



//----------------------------
//3.27.10 Texture Instructions
//----------------------------



instruction! { OpSampledImage, 5, 5,
// Create a sampler containing both a filter and texture.
    result_type: Id,         // Must be an OpTypeSampler whose Sampled Type,
                                            // Dim, Arrayed, Comparison, and
                                            // Multisampled operands all equal those of
                                            // this instruction's Sampler operand.
                                            // Further, the Result Type must have its
                                            // Content operand set to 2, indicating
                                            // both a texture and filter are present.
    result: Id,
    sampler: Id,            // Must be an object whose type is from an OpTypeSampler.
                                            // Its type must have its Content operand set to 0,
                                            // indicating a texture with no filter.
    filter: Id            // Must be an object whose type is OpTypeFilter.
}

// Sample a texture with an implicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleImplicitLod, 5, 65535,
    result_type: Id,         // Result Type's component type must be the same as
                                            // Sampled Type of Sampler's type. Result Type must
                                            // be scalar if the Sampler's type sets depth-comparison,
                                            // and must be a vector of four components if the
                                            // Sampler's type does not set depth-comparison.
    result: Id,
    sampler: Id,            // Must be an object whose type is from an OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
    coordinate: Id,         // A floating-point scalar or vector containing
                                            //(u[, v] ... [, array layer] [, Dref]) as needed
                                            // by the definiton of Sampler.
                                            // It may be a vector larger than needed, but all
                                            // unused components will appear after all used components.
    operands: UnsizedArray<Id>
}

// Sample a cube-map-array texture with depth
// comparison using an implicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleDrefImplicitLod, 6, 6,
    result_type: Id,         // Must be scalar of the same type as Sampled Type
                                            // of Sampler's type.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // It must be for a Cube-arrayed depth-comparison type.
    coordinate: Id,         // A vector of size 4 containing (u, v, w, array layer).
    dref: Id,               // The depth-comparison reference value.
    operands: UnsizedArray<Id>
}

// Sample a texture with a projective
// coordinate using an implicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleProjImplicitLod, 5, 6,
    result_type: Id,         // Result Type's component type must be the same as
                                            // Sampled Type of Sampler's type. Result Type must
                                            // be scalar if the Sampler's type sets depth-comparison,
                                            // and must be a vector of four components if the
                                            // Sampler's type does not set depth-comparison.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
    coordinate: Id,         // A floating-point vector of four components containing
                                            //(u [, v] [, Dref], q) or (u [, v] [,w], q), as
                                            // needed by the definiton of Sampler, with the q
                                            // component consumed for the projective division.
                                            // That is, the actual sample coordinate will be
                                            //(u/q[, v/q][, Dref/q]) or (u/q[, v/q][, w/q]), as
                                            // needed by the definiton of Sampler.
    operands: UnsizedArray<Id>
}

// Sample a texture with a projective
// coordinate using an implicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleProjDrefImplicitLod, 5, 6,
    result_type: Id,         // Result Type's component type must be the same as
                                            // Sampled Type of Sampler's type. Result Type must
                                            // be scalar if the Sampler's type sets depth-comparison,
                                            // and must be a vector of four components if the
                                            // Sampler's type does not set depth-comparison.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
    coordinate: Id,         // A floating-point vector of four components containing
                                            //(u [, v] [, Dref], q) or (u [, v] [,w], q), as
                                            // needed by the definiton of Sampler, with the q
                                            // component consumed for the projective division.
                                            // That is, the actual sample coordinate will be
                                            //(u/q[, v/q][, Dref/q]) or (u/q[, v/q][, w/q]), as
                                            // needed by the definiton of Sampler.
    dref: Id,               // The depth-comparison reference value.
    operands: UnsizedArray<Id>
}

// Sample a texture using an explicit
// level of detail.
//
// Capability: Shader
instruction! { OpImageSampleExplicitLod, 6, 6,
    result_type: Id,         // Result Type's component type must be the same as
                                            // Sampled Type of Sampler's type. Result Type must
                                            // be scalar if the Sampler's type sets depth-comparison,
                                            // and must be a vector of four components if the
                                            // Sampler's type does not set depth-comparison.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
    coordinate: Id,         // a floating-point scalar or vector containing
                                            //(u[, v] ... [, array layer] [, Dref]) as needed
                                            // by the definiton of Sampler.
                                            // It may be a vector larger than needed, but all
                                            // unused components will appear after all used components.
    operands: UnsizedArray<Id>
}


// Sample a cube-map-array texture with depth
// comparison using an explicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleDrefExplicitLod, 6, 6,
    result_type: Id,         // Must be scalar of the same type as Sampled Type
                                            // of Sampler's type.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // It must be for a Cube-arrayed depth-comparison type.
    coordinate: Id,         // A vector of size 4 containing (u, v, w, array layer).
    dref: Id,               // The depth-comparison reference value.
    operands: UnsizedArray<Id>
}

// Sample a texture with a projective
// coordinate using an explicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleProjExplicitLod, 5, 6,
    result_type: Id,         // Result Type's component type must be the same as
                                            // Sampled Type of Sampler's type. Result Type must
                                            // be scalar if the Sampler's type sets depth-comparison,
                                            // and must be a vector of four components if the
                                            // Sampler's type does not set depth-comparison.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
    coordinate: Id,         // A floating-point vector of four components containing
                                            //(u [, v] [, Dref], q) or (u [, v] [,w], q), as
                                            // needed by the definiton of Sampler, with the q
                                            // component consumed for the projective division.
                                            // That is, the actual sample coordinate will be
                                            //(u/q[, v/q][, Dref/q]) or (u/q[, v/q][, w/q]), as
                                            // needed by the definiton of Sampler.
    operands: UnsizedArray<Id>
}

// Sample a texture with a projective
// coordinate using an explicit level of detail.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
//
// Capability: Shader
instruction! { OpImageSampleProjDrefExplicitLod, 5, 6,
    result_type: Id,         // Result Type's component type must be the same as
                                            // Sampled Type of Sampler's type. Result Type must
                                            // be scalar if the Sampler's type sets depth-comparison,
                                            // and must be a vector of four components if the
                                            // Sampler's type does not set depth-comparison.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
    coordinate: Id,         // A floating-point vector of four components containing
                                            //(u [, v] [, Dref], q) or (u [, v] [,w], q), as
                                            // needed by the definiton of Sampler, with the q
                                            // component consumed for the projective division.
                                            // That is, the actual sample coordinate will be
                                            //(u/q[, v/q][, Dref/q]) or (u/q[, v/q][, w/q]), as
                                            // needed by the definiton of Sampler.
    dref: Id,               // The depth-comparison reference value.
    operands: UnsizedArray<Id>
}

// Fetch a single texel from a texture.
//
// Capability: Shader
instruction! { OpImageFetch, 5, 5,
    result_type: Id,         // Must be a vector of four components of the same
                                            // type as Sampled Type of Sampler's type.
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // It must have a Dim of 1D, 2D, or 3D.
                                            // It cannot have depth-comparison type
                                            // (the type's Compare operand must be 0).
    coordinate: Id,         // An integer scalar or vector containing
                                            // (u[, v] ... [, array layer]) as needed by the
                                            // definiton of Sampler.
    operands: UnsizedArray<Id>
}

// Gathers the requested component from
// four sampled texels.
//
// Capability: Shader
instruction! { OpImageGather, 6, 6,
    // Must be a vector of four components of the same
    // type as Sampled Type of Sampler's type.
    // The result has one component per gathered texel.
    result_type: Id,
    result: Id,
    // Must be an object of a type made by OpSampledImage.
    // Its OpTypeImage must have a Dim of 2D, Cube, or Rect.
    sampled_image: Id,
    // A floating-point scalar or vector containing
    // (u[, v] ... [, array layer] [, Dref]) as needed
    // by the definiton of Sampler.
    coordinate: Id,
    // component number that will be gathered from all
    // four texels. It must be 0, 1, 2 or 3.
    component: Id,
    operands: UnsizedArray<Id>
}

// Gathers the requested component from
// four sampled texels.
//
// Capability: Shader
instruction! { OpImageDrefGather, 6, 6,
    // Must be a vector of four components of the same
    // type as Sampled Type of Sampler's type.
    // The result has one component per gathered texel.
    result_type: Id,
    result: Id,
    // Must be an object of a type made by OpSampledImage.
    // Its OpTypeImage must have a Dim of 2D, Cube, or Rect.
    sampled_image: Id,
    // A floating-point scalar or vector containing
    // (u[, v] ... [, array layer] [, Dref]) as needed
    // by the definiton of Sampler.
    coordinate: Id,
    // component number that will be gathered from all
    // four texels. It must be 0, 1, 2 or 3.
    component: Id,
    // The depth-comparison reference value.
    dref: Id,
    operands: UnsizedArray<Id>
}

// Read a texel from an image without a sampler.
instruction! { OpImageRead, 5, 5,
    // Result Type must be a scalar or vector of floating-point type or integer type.
    // Its components must be the same as Sampled Type of the OpTypeImage (unless that underlying Sampled Type is OpTypeVoid).
    result_type: Id,
    result: Id,
    // Image must be an object whose type is OpTypeImage with a Sampled operand of 0 or 2.
    image: Id,
    // Coordinate is an integer scalar or vector containing non-parametric texel coordinates (u[, v] . . . [, array layer]) as needed
    // by the definiton of Image. If the coordinates are outside the image, the memory location that is accessed is undefined.
    coordinate: Id
}

// Writes a texel from an image without a sampler.
instruction! { OpImageWrite, 5, 5,
    // Result Type must be a scalar or vector of floating-point type or integer type.
    // Its components must be the same as Sampled Type of the OpTypeImage (unless that underlying Sampled Type is OpTypeVoid).
    result_type: Id,
    result: Id,
    // Image must be an object whose type is OpTypeImage with a Sampled operand of 0 or 2.
    image: Id,
    // Coordinate is an integer scalar or vector containing non-parametric texel coordinates (u[, v] . . . [, array layer]) as needed
    // by the definiton of Image. If the coordinates are outside the image, the memory location that is accessed is undefined.
    coordinate: Id
}

// Query the image format of an image created with an Unknown Image Format.
//
// Capability: Kernel
instruction! { OpImageQueryDim, 4, 4,
    // Result Type must be a scalar integer type. The resulting value is an enumerant from Image Channel Data Type.
    result_type: Id,
    result: Id,
    // Image must be an object whose type is OpTypeSampledImage or OpTypeImage.
    image: Id
}

// Query the image format of an image created with an Unknown Image Format.
//
// Capability: Kernel
instruction! { OpImageQueryFormat, 4, 4,
    // Result Type must be a scalar integer type. The resulting value is an enumerant from Image Channel Data Type.
    result_type: Id,
    result: Id,
    // Image must be an object whose type is OpTypeSampledImage or OpTypeImage.
    image: Id
}

// Query the image order of an image created with an Unknown Image Format.
//
// Capability: Kernel
instruction! { OpImageQueryOrder, 4, 4,
    // Result Type must be a scalar integer type. The resulting value is an enumerant from Image Channel Order.
    result_type: Id,
    result: Id,
    // Image must be an object whose type is OpTypeSampledImage or OpTypeImage.
    image: Id
}

// Query the dimensions of the texture for
// Sampler for mipmap level for Level of Detail.
//
// Capability: Shader
instruction! { OpImageQuerySizeLod, 5, 5,

    result_type: Id,         // Must be an integer type scalar or vector.
                                            // The number of components must be
                                            // 1 for 1D Dim,
                                            // 2 for 2D, and Cube Dimensionalities,
                                            // 3 for 3D Dim,
                                            // plus 1 more if the sampler type is arrayed.
                                            // This vector is filled in with
                                            // (width[, height][, depth][, elements])
                                            // where elements is the number of layers
                                            // in a texture array, or the number of cubes in
                                            // a cube-map array.

    result: Id,

    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // Sampler must have a type with Dim
                                            // of 1D, 2D, 3D, or Cube.
                                            // Sampler cannot have a multisampled type.
                                            // See OpImageQuerySize for querying texture
                                            // types lacking level of detail.

    levelOfDetail: Id     // explicitly controls the level of detail
                                            // used when sampling.
}

instruction! { OpImageQuerySize, 4, 4,
// Query the dimensions of the texture for
// Sampler, with no level of detail.
//
// Capability: Shader
    result_type: Id,         // Must be an integer type scalar or vector.
                                            // The number of components must be
                                            // 1 for Buffer Dim,
                                            // 2 for 2D and Rect Dimensionalities,
                                            // plus 1 more if the sampler type is arrayed.
                                            // This vector is filled in with
                                            // (width[, height][,elements]) where elements
                                            // is the number of layers in a texture array.
    result: Id,
    sampler: Id            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // Sampler must have a type with Dim of
                                            // Rect or Buffer, or be multisampled 2D.
                                            // Sampler cannot have a texture with levels of
                                            // detail; there is no implicit level-of-detail
                                            // consumed by this instruction.
                                            // See OpImageQuerySizeLod for querying textures
                                            // having level of detail.
}

instruction! { OpImageQueryLod, 5, 5,
// Query the mipmap level and the level
// of detail for a hypothetical sampling
// of Sampler at Coordinate using an
// implicit level of detail.
//
// If called on an incomplete texture,
// the results are undefined.
//
// This instruction is only allowed under the
// Fragment Execution Model. In addition, it
// consumes an implicit derivative that can
// be affected by code motion.
// Capability: Shader
    result_type: Id,         // Must be a two-component floating-point type vector.
                                            // The first component of the result will contain
                                            // the mipmap array layer. The second component
                                            // of the result will contain the implicit level
                                            // of detail relative to the base level.
                                            // TBD: Does this need the GLSL pseudo code
                                            // for computing array layer and LoD?
    result: Id,
    sampler: Id,            // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // Sampler must have a type with Dim
                                            // of 1D, 2D, 3D, or Cube.
    coordinate: Id         // A floating-point scalar or vector containing
                                            // (u[, v] ... [, array layer]) as needed by the
                                            // definiton of Sampler.
}

instruction! { OpImageQueryLevels, 4, 4,
// Query the number of mipmap levels
// accessible through Sampler.
//
// TBD: The value zero will be returned
// if no texture or an incomplete texture
// is associated with Sampler.
//
// Capability: Shader
    result_type: Id,         // Must be a scalar integer type.
                                            // The result is the number of mipmap levels,
                                            // as defined by the API specification.
    result: Id,
    sampler: Id             // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // Sampler must have a type with Dim
                                            // of 1D, 2D, 3D, or Cube.
}

instruction! { OpImageQuerySamples, 4, 4,
// Query the number of samples available
// per texel fetch in a multisample texture.
//
// Capability: Shader
    result_type: Id,         // Must be a scalar integer type.
                                            // The result is the number of samples.
    result: Id,
    sampler: Id             // Must be an object of a type made by OpTypeSampler.
                                            // Its type must have its Content operand set to 2,
                                            // indicating both a texture and a filter.
                                            // Sampler must have a type with Dim
                                            // of 2D and be a multisample texture.
}



///-------------------------------
///3.27.11 Conversion Instructions
///-------------------------------



instruction! { OpConvertFToU, 4, 4,
// Convert(value preserving) Float Value from
// floating point to unsigned integer,
// with round toward 0.0.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
//
// Result Type cannot be a signed integer type.
    result_type: Id,
    result: Id,
    float_value: Id
}

instruction! { OpConvertFToS, 4, 4,
// Convert(value preserving) Float Value from
// floating point to signed integer,
// with round toward 0.0.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
    result_type: Id,
    result: Id,
    float_value: Id
}

instruction! { OpConvertSToF, 4, 4,
// Convert(value preserving) Signed Value
// from signed integer to floating point.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
    result_type: Id,
    result: Id,
    signedValue: Id
}

instruction! { OpConvertUToF, 4, 4,
// Convert(value preserving) Unsigned value
// from unsigned integer to floating point.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
    result_type: Id,
    result: Id,
    unsignedValue: Id
}

instruction! { OpUConvert, 4, 4,
// Convert(value preserving) the width of
// Unsigned value. This is either
// a truncate or a zero extend.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
// The widths of the components of the
// operand and the Result Type must be
// different. Result Type cannot be a
// signed integer type.
    result_type: Id,
    result: Id,
    unsignedValue: Id
}

instruction! { OpSConvert, 4, 4,
// Convert(value preserving) the width of
// Signed Value. This is either a
// truncate or a sign extend.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
// The widths of the components of the
// operand and the Result Type must be different.
    result_type: Id,
    result: Id,
    signedValue: Id
}

instruction! { OpFConvert, 4, 4,
// Convert(value preserving) the width
// of Float Value.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
// The widths of the components of the
// operand and the Result Type must be different.
    result_type: Id,
    result: Id,
    float_value: Id
}

// Quantize a floating-point value to a what is expressable by a 16-bit floating-point value.
instruction! { OpQuantizeToF16, 4, 4,
    result_type: Id,
    result: Id,
    value: Id
}

instruction! { OpConvertPtrToU, 4, 4,
// Convert Pointer to an unsigned
// integer type. A Result Type width
// larger than the width of Pointer
// will zero extend. A Result Type
// smaller than the width of Pointer
// will truncate. For same-width
// source and target, this is the
// same as OpBitCast.
//
// Capability: Addr
    result_type: Id,         // Cannot be a signed integer type.
    result: Id,
    pointer: Id
}

instruction! { OpConvertUToPtr, 4, 4,
// Converts Integer value to a pointer.
// A Result Type width smaller than the
// width of Integer value pointer will
// truncate. A Result Type width larger
// than the width of Integer value
// pointer will zero extend. For
// same-width source and target,
// this is the same as OpBitCast.
//
// Capability: Addr
    result_type: Id,
    result: Id,
    integerValue: Id
}

instruction! { OpPtrCastToGeneric, 4, 4,
// Converts Source pointer to a pointer
// value pointing to storage class Generic.
// Source pointer must point to storage
// class WorkgroupLocal, WorkgroupGlobal
// or Private. Result Type must be a
// pointer type pointing to storage
// class Generic.
//
// Result Type and Source pointer
// Must point to the same type.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    sourcePointer: Id
}

instruction! { OpGenericCastToPtr, 4, 4,
// Converts Source pointer to a
// non-Generic storage-class pointer
// value. Source pointer must point
// to Generic.
//
// Result Type and Source pointer
// Must point to the same type.
//
// Capability: Kernel
    result_type: Id,         // Must be a pointer type pointing to
                                            // WorkgroupLocal, WorkgroupGlobal or
                                            // Private.
    result: Id,
    sourcePointer: Id
}

instruction! { OpBitcast, 4, 4,
// Bit-pattern preserving type conversion
// for Numerical-type or pointer-type
// vectors and scalars.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of components.
    result_type: Id,         // Must be different than the type of Operand.
                                            // Both Result Type and the type of Operand
                                            // Must be Numerical-types or pointer types.
                                            // The components of Operand and Result
                                            // Type must be same bit width.
    result: Id,
    operand: Id           // The bit pattern whose type will change.
}

// Attempts to explicitly convert Source
// pointer to storage storage-class
// pointer value. Source pointer must
// point to Generic. If the cast cast
// fails, the instruction returns an
// OpConstantNullPointer in storage
// Storage Class.
//
// Result Type and Source pointer
// Must point to the same type.
//
// Capability: Kernel
instruction! { OpGenericCastToPtrExplicit, 4, 4,
    result_type: Id,         // Must be a pointer type pointing to storage
                                            //Storage Class. Storage can be one of the
                                            // following literal values:
                                            // WorkgroupLocal, WorkgroupGlobal or Private.
    result: Id,
    source_pointer: Id,
    storage: StorageClass
}

instruction! { OpSatConvertSToU, 4, 4,
// Convert the Signed Value from signed
// integer to unsigned integer.
// Converted values outside the
// representable range of Result Type
// are clamped to the nearest
// representable value of Result Type.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of
// components.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    signedValue: Id
}

instruction! { OpSatConvertUToS, 4, 4,
// Convert Unsigned Value from unsigned
// integer to signed integer. Converted
// values outside the representable
// range of Result Type are clamped to
// the nearest representable value
// of Result Type.
//
// Results are computed per component.
// The operand's type and Result Type
// Must have the same number of
// components.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    unsignedValue: Id
}



///-------------------------------
///3.27.12 Composite Instructions
///-------------------------------



instruction! { OpVectorExtractDynamic, 5, 5,
// Read a single, dynamically selected,
// component of a vector.
//
// The value read is undefined if Index's
// value is less than zero or greater than
// or equal to the number of components in
// Vector.
//
// The Result Type must be the same type as
// the type of Vector.
    result_type: Id,
    result: Id,
    vector: Id,             // Must be a vector type and is the vector from which to read the component.
    index: Id               // Must be a scalar-integer 0-based index of which component to read.
}

instruction! { OpVectorInsertDynamic, 6, 6,
// Write a single, variably selected, component into a vector.
//
// What memory is written is undefined if Index's value is less than zero or greater than or equal to the number of
// components in Vector.
//
// The Result Type must be the same type as the type of Vector.
    result_type: Id,
    result: Id,
    vector: Id,             // Must be a vector type and is the vector that the non-written components will be taken from.
    component: Id,
    index: Id               // Must be a scalar-integer 0-based index of which component to read.
}

instruction! { OpVectorShuffle, 5, 65535,
// Select arbitrary components from two vectors to make a new vector.
//
// Note: A vector "swizzle" can be done by using the vector for both Vector operands, or using an OpUndef for one of the
// Vector operands.
    result_type: Id,         // Must be a vector of the same component type as the Vector operands' component type. The number of
                                            // components in Result Type must be the same as the number of Component operands.
    result: Id,
    vector1: Id, vector2: Id,   // Vector 1 and Vector 2 are logically concatenated, forming a single vector with Vector 1's components appearing before
                                            // Vector 2's. The components of this logical vector are logically numbered with a single consecutive set of numbers from 0
                                            // to one less than the total number of components. These two vectors must be of the same component type, but do not have
                                            // to have the same number of components.

    components: UnsizedArray<LiteralNumber>      // Components are these logical numbers (see above), selecting which of the logically numbered components form the result.
                                            // They can select the components in any order and can repeat components. The first component of the result is selected by
                                            // the first Component operand, the second component of the result is selected by the second Component operand, etc.
}

instruction! { OpCompositeConstruct, 3, 65535,
//Construct a new composite object from a set of constituent objects that will fully form it.
    result_type: Id,         // Must be a composite type, whose top-level members/elements/components/columns have the same type as the
                                            // types of the operands, with one exception. The exception is that for constructing a vector, the operands may also be
                                            // vectors with the same component type as the Result Type component type. When constructing a vector, the total number of
                                            // components in all the operands must equal the number of components in Result Type.
    result: Id,
    constituents: UnsizedArray<Id>    // Constituents will become members of a structure, or elements of an array, or components of a vector, or columns of a
                                            // matrix. There must be exactly one Constituent for each top-level member / element / component / column of the result, with
                                            // one exception. The exception is that for constructing a vector, a contiguous subset of the scalars consumed can be
                                            // represented by a vector operand instead. The Constituents must appear in the order needed by the definition of the type of
                                            // the result. When constructing a vector, there must be at least two Constituent operands.
}

// Extract a part of a composite object.
instruction! { OpCompositeExtract, 4, 65535,
    // Must be the type of object selected by the last provided index. The instruction result is the extracted object.
    result_type: Id,
    result: Id,
    // Composite in the composite to extract from.
    composite: Id,
    // Indexes walk the type hierarchy, down to component granularity. All indexes must be in bounds.
    indexes: UnsizedArray<LiteralNumber>
}

instruction! { OpCompositeInsert, 5, 65535,
// Insert into a composite object.
    result_type: Id,         // Must be the same type as Composite, and the instruction result is a modified version of Composite.
    result: Id,
    object: Id,             // The object to insert.
    composite: Id,          // Composite in the composite to insert into.
    indexes: UnsizedArray<LiteralNumber>         // Indexes walk the type hierarchy to the desired depth, potentially down to component granularity. All indexes must be in bounds.
}

instruction! { OpCopyObject, 4, 4,
// Make a copy of Operand. There are no dereferences involved.
    result_type: Id,         // Must match Operand type. There are no other restrictions on the types.
    result: Id,
    operand: Id
}

instruction! { OpTranspose, 4, 4,
// Transpose a matrix.
//
// Capability: Matrix
    result_type: Id,         // Must be an <id> from an OpTypeMatrix instruction, where the number of
                                            // columns and the column size is the reverse of those of the type of Matrix.
    result: Id,
    matrix: Id            // Must be an intermediate <id> whose type comes from an OpTypeMatrix instruction.
}



///-------------------------------
///3.27.13 Arithmetic Instructions
///-------------------------------



instruction! { OpSNegate, 4, 4,
// Signed-integer subtract of Operand from zero. The operand's type and Result Type must both be
// scalars or vectors of integer types with the same number of components and the same component
// widths. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand: Id
}

instruction! { OpFNegate, 4, 4,
// Floating-point subtract of Operand from zero. The operand's type and Result
// Type must both be scalars or vectors of floating-point types with the same number
// of components and the same component widths.
    result_type: Id,
    result: Id,
    operand: Id
}

instruction! { OpNot, 4, 4,
// Complement the bits of Operand. The operand type and Result Type
// Must be scalars or vectors of integer types with the same number of
// components and same component widths.
    result_type: Id,
    result: Id,
    operand: Id
}

instruction! { OpIAdd, 5, 5,
// Integer addition of Operand 1 and Operand 2. The operands' types and Result Type must all be
// scalars or vectors of integer types with the same number of components and the same component
// widths. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpFAdd, 5, 5,
// Floating-point addition of Operand 1 and Operand 2. The operands' types and Result
// Type must all be scalars or vectors of floating-point types with the same number of
// components and the same component widths.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpISub, 5, 5,
// Integer subtraction of Operand 2 from Operand 1. The operands' types and Result Type must all be
// scalars or vectors of integer types with the same number of components and the same component
// widths. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpFSub, 5, 5,
// Floating-point subtraction of Operand 2 from Operand 1. The operands' types and
// Result Type must all be scalars or vectors of floating-point types with the same number
// of components and the same component widths.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpIMul, 5, 5,
// Integer multiplication of Operand 1 and Operand 2. The operands' types and Result Type must all be
// scalars or vectors of integer types with the same number of components and the same component
// widths. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpFMul, 5, 5,
// Floating-point multiplication of Operand 1 and Operand 2. The operands' types and
// Result Type must all be scalars or vectors of floating-point types with the same number
// of components and the same component widths.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpUDiv, 5, 5,
// Unsigned-integer division of Operand 1 divided by Operand 2. The operands' types and Result Type must all be scalars or
// vectors of integer types with the same number of components and the same component widths. The operands' types and
// Result Type cannot be signed types. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpSDiv, 5, 5,
// Signed-integer division of Operand 1 divided by Operand 2. The operands' types and Result Type must all be scalars or
// vectors of integer types with the same number of components and the same component widths. Works with any mixture of
// signedness. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpFDiv, 5, 5,
// Floating-point division of Operand 1 divided by Operand 2. The operands' types and Result Type must all be
// scalars or vectors of floating-point types with the same number of components and the same component widths.
// The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpUMod, 5, 5,
// Unsigned modulo operation of Operand 1 modulo Operand 2. The operands' types and Result Type must all be scalars or
// vectors of integer types with the same number of components and the same component widths. The operands' types and
// Result Type cannot be signed types. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpSRem, 5, 5,
// Signed remainder operation of Operand 1 divided by Operand 2. The sign of a non-0 result comes from Operand 1. The
// operands' types and Result Type must all be scalars or vectors of integer types with the same number of components and
// the same component widths. Works with any mixture of signedness. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpSMod, 5, 5,
// Signed modulo operation of Operand 1 modulo Operand 2. The sign of a non-0 result comes from Operand 2. The
// operands' types and Result Type must all be scalars or vectors of integer types with the same number of components and
// the same component widths. Works with any mixture of signedness. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpFRem, 5, 5,
// Floating-point remainder operation of Operand 1 divided by Operand 2. The sign of a non-0 result comes from Operand
// 1. The operands' types and Result Type must all be scalars or vectors of floating-point types with the same number of
// components and the same component widths. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpFMod, 5, 5,
// Floating-point modulo operation of Operand 1 modulo Operand 2. The sign of a non-0 result comes from Operand 2. The
// operands' types and Result Type must all be scalars or vectors of floating-point types with the same number of components
// and the same component widths. The resulting value is undefined if Operand 2 is 0.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpVectorTimesScalar, 5, 5,
// Scale a floating-point vector.
    result_type: Id,         // Must be the same as the type of Vector.
    result: Id,
    vector: Id,             // Must have a floating-point vector type.
    scalar: Id              // Must be a floating-point scalar.
}

instruction! { OpMatrixTimesScalar, 5, 5,
// Scale a floating-point matrix.
//
// Capability: Matrix
    result_type: Id,         // Must be the same as the type of Matrix.
    result: Id,
    matrix: Id,             // Must have a floating-point matrix type.
    scalar: Id              // Must have a floating-point scalar type.
}

instruction! { OpVectorTimesMatrix, 5, 5,
// Linear-algebraic Vector X Matrix.
//
// Capability: Matrix
    result_type: Id,         // Must be a vector whose size is the number of columns in the matrix.
    result: Id,
    vector: Id,             // Must have a floating-point vector type.
    matrix: Id              // Must have a floating-point matrix type.
}

instruction! { OpMatrixTimesVector, 5, 5,
// Linear-algebraic Vector X Matrix.
//
// Capability: Matrix
    result_type: Id,         // Must be a vector whose size is the number of rows in the matrix.
    result: Id,
    matrix: Id,             // Must have a floating-point matrix type.
    vector: Id              // Must have a floating-point vector type.
}

instruction! { OpMatrixTimesMatrix, 5, 5,
// Linear-algebraic multiply of LeftMatrix X RightMatrix.
//
// Capability: Matrix
    result_type: Id,         // Must be a matrix whose number of columns is the number of columns in
                                            // RightMatrix and whose number of rows is the number of rows of LeftMatrix
    result: Id,
    leftMatrix: Id,
    rightMatrix: Id        // LeftMatrix and RightMatrix must both have a floating-point matrix type.
                                            // The number of columns of LeftMatrix must equal the number of rows of RightMatrix.
}

instruction! { OpOuterProduct, 5, 5,
// Linear-algebraic outer product of Vector 1 and Vector 2.
//
// The operands' types must be floating-point vectors with the same component type and the same
// number of components.
    result_type: Id,         // Must be a matrix type. Its number of columns must equal the number of components
                                            // in Vector 2. The vector type of its columns must be the same as the type of Vector 1.
    result: Id,
    vector1: Id, vector2: Id
}

instruction! { OpDot, 5, 5,
// Dot product of Vector 1 and Vector 2.
//
// The operands' types must be floating-point vectors with the same component type and the same
// number of components.
    result_type: Id,         // Must be a scalar of the same type as the operands' component type.
    result: Id,
    vector1: Id, vector2: Id
}

instruction! { OpShiftRightLogical, 5, 5,
// Shift the bits in Operand 1 right by the number of bits specified in Operand 2. The most-significant bits will be zero filled.
// Operand 2 is consumed as an unsigned integer. The result is undefined if Operand 2 is greater than the bit width of the
// components of Operand 1.
//
// The number of components and bit width of Result Type must match those of Operand 1 type. All types must be integer
// types. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpShiftRightArithmetic, 5, 5,
// Shift the bits in Operand 1 right by the number of bits specified in Operand 2. The most-significant bits will be filled with
// the sign bit from Operand 1. Operand 2 is treated as unsigned. The result is undefined if Operand 2 is greater than the bit
// width of the components of Operand 1.
//
// The number of components and bit width of Result Type must match those Operand 1 type. All types must be integer
// types. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpShiftLeftLogical, 5, 5,
// Shift the bits in Operand 1 left by the number of bits specified in Operand 2. The least-significant bits will be zero filled.
//
// Operand 2 is treated as unsigned. The result is undefined if Operand 2 is greater than the bit width of the components of
// Operand 1.
// The number of components and bit width of Result Type must match those Operand 1 type. All types must be integer
// types. Works with any mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpBitwiseOr, 5, 5,
// Result is 1 if either Operand 1 or Operand 2 is 1. Result is 0 if both Operand 1 and Operand 2 are 0.
//
// Results are computed per component, and within each component, per bit. The operands' types and Result Type must all be
// scalars or vectors of integer types with the same number of components and the same component widths. Works with any
// mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpBitwiseXor, 5, 5,
// Result is 1 if exactly one of Operand 1 or Operand 2 is 1. Result is 0 if Operand 1 and Operand 2 have the same value.
//
// Results are computed per component, and within each component, per bit. The operands' types and Result Type must all be
// scalars or vectors of integer types with the same number of components and the same component widths. Works with any
// mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}

instruction! { OpBitwiseAnd, 5, 5,
// Result is 1 if both Operand 1 and Operand 2 are 1. Result is 0 if either Operand 1 or Operand 2 are 0.
//
// Results are computed per component, and within each component, per bit. The operands' types and Result Type must all be
// scalars or vectors of integer types with the same number of components and the same component widths. Works with any
// mixture of signedness.
    result_type: Id,
    result: Id,
    operand1: Id, operand2: Id
}



///-------------------------------------------
///3.27.14 Relational and Logical Instructions
///-------------------------------------------



instruction! { OpAny, 4, 4,
// Result is true if any component of Vector is true, otherwise result is false.
    result_type: Id,         // Must be a Boolean type scalar.
    result: Id,
    vector: Id            // Must be a vector of Boolean type.
}

instruction! { OpAll, 4, 4,
// Result is true if all components of Vector are true, otherwise result is false.
    result_type: Id,         // Must be a Boolean type scalar.
    result: Id,
    vector: Id            // Must be a vector of Boolean type.
}

instruction! { OpIsNan, 4, 4,
// Result is true if x is an IEEE NaN, otherwise result is false.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operand. Results are computed per component. The operand's type and Result Type must have the same number of components.
    result: Id,
    x: Id
}

instruction! { OpIsInf, 4, 4,
// Result is true if x is an IEEE Inf, otherwise result is false
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operand. Results are computed per component. The operand's type and Result Type must have the same number of components.
    result: Id,
    x: Id
}

instruction! { OpIsFinite, 4, 4,
// Result is true if x is an IEEE finite number, otherwise result is false.
//
// Capability: Kernel
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operand. Results are computed per component. The operand's type and Result Type must have the same number of components.
    result: Id,
    x: Id
}

instruction! { OpIsNormal, 4, 4,
// Result is true if x is an IEEE normal number, otherwise result is false.
//
// Capability: Kernel
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operand. Results are computed per component. The operand's type and Result Type must have the same number of components.
    result: Id,
    x: Id
}

instruction! { OpSignBitSet, 4, 4,
// Result is true if x has its sign bit set, otherwise result is false.
//
// Capability: Kernel
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operand. Results are computed per component. The operand's type and Result Type must have the same number of components.
    result: Id,
    x: Id
}

instruction! { OpLessOrGreater, 5, 5,
// Result is true if x < y or x > y, where IEEE comparisons are used, otherwise result is false.
//
// Capability: Kernel
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    x: Id,
    y: Id
}

instruction! { OpOrdered, 5, 5,
// Result is true if both x == x and y == y are true, where IEEE comparison is used, otherwise result is false.
//
// Capability: Kernel
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    x: Id,
    y: Id
}

instruction! { OpUnordered, 5, 5,
// Result is true if either x or y is an IEEE NaN, otherwise result is false.
//
// Capability: Kernel
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    x: Id,
    y: Id
}

instruction! { OpLogicalOr, 5, 5,
// Result is true if either Operand 1 or Operand 2 is true. Result is false if both Operand 1 and Operand 2 are false.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    operand1: Id, operand2: Id           // Operand 1 and Operand 2 must both be scalars or vectors of Boolean type.
}

// Result is true if exactly one of Operand 1 or Operand 2 is true. Result is false if Operand 1 and Operand 2 have the same value.
instruction! { OpLogicalNotEqual, 5, 5,
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    operand1: Id, operand2: Id           // Operand 1 and Operand 2 must both be scalars or vectors of Boolean type.
}

// ==
instruction! { OpLogicalEqual, 5, 5,
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    operand1: Id, operand2: Id           // Operand 1 and Operand 2 must both be scalars or vectors of Boolean type.
}

instruction! { OpLogicalAnd, 5, 5,
// Result is true if both Operand 1 and Operand 2 are true. Result is false if either Operand 1 or Operand 2 are false.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result: Id,
    operand1: Id, operand2: Id           // Operand 1 and Operand 2 must both be scalars or vectors of Boolean type.
}

instruction! { OpLogicalNot, 4, 4,
    // Must be a scalar or vector of Boolean type, with the same number of components as the operands. Results are computed per component. The operands' types and Result Type must all have the same number of components.
    result_type: Id,
    // Result is true if Operand is false. Result is false if Operand is true.
    result: Id,
    operand: Id
}

instruction! { OpSelect, 6, 6,
// Select between two objects. Results are computed per component.
//
// Result Type, the type of Object 1, and the type of Object 2 must all be the same.
    result_type: Id,
    result: Id,
    condition: Id,          // Must be a Boolean type scalar or vector.
                                            // Must have the same number of components as the operands.
    object1: Id,            // Object 1 is selected as the result if Condition is true.
    object2: Id             // Object 2 is selected as the result if Condition is false.
}

instruction! { OpIEqual, 5, 5,
// Integer comparison for equality.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFOrdEqual, 5, 5,
// Floating-point comparison for being ordered and equal.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFUnordEqual, 5, 5,
// Floating-point comparison for being unordered or equal.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpINotEqual, 5, 5,
// Integer comparison for inequality.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFOrdNotEqual, 5, 5,
// Floating-point comparison for being ordered and not equal.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFUnordNotEqual, 5, 5,
// Floating-point comparison for being unordered or not equal.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpULessThan, 5, 5,
// Unsigned-integer comparison if Operand 1 is less than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpSLessThan, 5, 5,
// Signed-integer comparison if Operand 1 is less than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFOrdLessThan, 5, 5,
// Floating-point comparison if operands are ordered and Operand 1 is less than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFUnordLessThan, 5, 5,
// Floating-point comparison if operands are unordered or Operand 1 is less than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpUGreaterThan, 5, 5,
// Unsigned-integer comparison if Operand 1 is greater than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpSGreaterThan, 5, 5,
// Signed-integer comparison if Operand 1 is greater than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFOrdGreaterThan, 5, 5,
// Floating-point comparison if operands are ordered and Operand 1 is greater than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFUnordGreaterThan, 5, 5,
// Floating-point comparison if operands are unordered or Operand 1 is greater than Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpULessThanEqual, 5, 5,
// Unsigned-integer comparison if Operand 1 is less than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpSLessThanEqual, 5, 5,
// Signed-integer comparison if Operand 1 is less than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFOrdLessThanEqual, 5, 5,
// Floating-point comparison if operands are ordered and Operand 1 is less than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFUnordLessThanEqual, 5, 5,
// Floating-point comparison if operands are unordered or Operand 1 is less than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpUGreaterThanEqual, 5, 5,
// Unsigned-integer comparison if Operand 1 is greater than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpSGreaterThanEqual, 5, 5,
// Signed-integer comparison if Operand 1 is greater than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFOrdGreaterThanEqual, 5, 5,
// Floating-point comparison if operands are ordered and Operand 1 is greater than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}

instruction! { OpFUnordGreaterThanEqual, 5, 5,
// Floating-point comparison if operands are unordered or Operand 1 is greater than or equal to Operand 2.
    result_type: Id,         // Must be a scalar or vector of Boolean type, with the same number of components as the operands.
    result: Id,
    operand1: Id,
    operand2: Id
}



///-------------------------------
///3.27.15 Derivative Instructions
///-------------------------------



instruction! { OpDPdx, 4, 4,
// Same result as either OpDPdxFine or OpDPdxCoarse on P. Selection of which one is
// based on external factors.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpDPdy, 4, 4,
// Same result as either OpDPdyFine or OpDPdyCoarse on P. Selection of which one is
// based on external factors.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpFwidth, 4, 4,
// Result is the same as computing the sum of the absolute values of OpDPdx and
// OpDPdy on P.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpDPdxFine, 4, 4,
// Result is the partial derivative of P with respect to the window x coordinate. Will use local
// differencing based on the value of P for the current fragment and its immediate
// neighbor(s).
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpDPdyFine, 4, 4,
// Result is the partial derivative of P with respect to the window y coordinate. Will use local
// differencing based on the value of P for the current fragment and its immediate
// neighbor(s).
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpFwidthFine, 4, 4,
// Result is the same as computing the sum of the absolute values of OpDPdxFine and
// OpDPdyFine on P.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpDPdxCoarse, 4, 4,
// Result is the partial derivative of P with respect to the window x coordinate. Will use
// local differencing based on the value of P for the current fragment's neighbors, and will
// possibly, but not necessarily, include the value of P for the current fragment. That is, over
// a given area, the implementation can compute x derivatives in fewer unique locations
// than would be allowed for OpDPdxFine.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpDPdyCoarse, 4, 4,
// Result is the partial derivative of P with respect to the window y coordinate. Will use
// local differencing based on the value of P for the current fragment's neighbors, and will
// possibly, but not necessarily, include the value of P for the current fragment. That is, over
// a given area, the implementation can compute y derivatives in fewer unique locations
// than would be allowed for OpDPdyFine.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}

instruction! { OpFwidthCoarse, 4, 4,
// Result is the same as computing the sum of the absolute values of OpDPdxCoarse and
// OpDPdyCoarse on P.
//
// Capability: Shader
    result_type: Id,         // Must be the same as the type of P. This type must be a floating-point scalar or floating-point vector.
    result: Id,
    p: Id                 // the value to take the derivative of.
}



///---------------------------------
///3.27.16 Flow-Control Instructions
///---------------------------------

#[derive(Debug)]
pub struct PhiPair {
    variable: Id,
    parent: Id
}

// The SSA phi function. Operands are pairs(<id> of variable, <id> of
// parent block).All variables must have a type matching Result Type.
instruction! { OpPhi, 3, 65535,
    result_type: Id,
    result: Id,
    pairs: [PhiPair; 1]
}

// Declare and control a structured control-flow loop construct.
//
// See Structured Control Flow for more detail.
instruction! { OpLoopMerge, 3, 3,
    // The label of the merge block for this structured loop construct.
    label: Id,
    loop_control_shift: LoopControlShift
}

// Declare and control a structured control-flow selection construct, used with OpBranchConditional or OpSwitch.
//
// See Structured Control Flow for more detail.
instruction! { OpSelectionMerge, 3, 3,
    // The label of the merge block for this structured selection construct.
    label: Id,
    selection_control_shift: SelectionControlShift
}

// The block label instruction : Any reference to a block is through the Result
///<id> of its label.
//
// Must be the first instruction of any block, and appears only as the first
// instruction of a block.
instruction! { OpLabel, 3, 3,
    result: Id
}

// Unconditional branch to Target Label.
//
// This instruction must be the last instruction in a block.
instruction! { OpBranch, 3, 3,
    target_label: Id       // Must be the Result <id> of an OpLabel instruction in the current function.
}

// If Condition is true, branch to True Label, otherwise branch to False Label.
instruction! { OpBranchConditional, 4, 65535,
    condition: Id,          // Must be a Boolean type scalar.
    true_label: Id,          // Must be an OpLabel in the current function.
    false_label: Id,         // Must be an OpLabel in the current function.
    branch_weights: [LiteralNumber; 1]   // Branch weights are unsigned 32-bit integer literals. There must be either no Branch Weights or exactly two branch weights.
                                            // If present, the first is the weight for branching to True Label, and the second is the weight for branching to False Label.
                                            // The implied probability that a branch is taken is its weight divided by the sum of the two Branch weights.
                                            // This instruction must be the last instruction in a block.
}

#[derive(Debug)]
pub struct SwitchCase {
    selector: LiteralNumber,
    label: Id
}

instruction! { OpSwitch, 3, 65535,
// Multi-way branch to one of the operand label <id>.
//
// This instruction must be the last instruction in a block.
    selector: Id,           // Must be a scalar integer type. It will be compared for equality to the Target literals.
    default_label: Id,       // Must be the <id> of a label. If Selector does not equal any of the Target literals, control flow will branch to the Default label <id>.
    target: [SwitchCase; 1]          // Target must be alternating scalar-integer literals and the <id> of a label. If Selector equals one of the literals, control flow
                                            // will branch to the following label <id>.It is invalid for any two Target literals to be equal to each other. If Target is not
                                            // present, control flow will branch to the Default label <id>.
}

// Fragment shader discard.
// This instruction must be the last instruction in a block.
// Capability: Shader
instruction! { OpKill, 1, 1 }

// Return with no value from a function with void return type.
// This instruction must be the last instruction in a block.
instruction! { OpReturn, 1, 1 }

// Return a value from a function.
//
// This instruction must be the last instruction in a block.
instruction! { OpReturnValue, 2, 2,
    value: Id              // the value returned, by copy, and must match the Return Type operand of the OpTypeFunction type of the OpFunction body this return instruction is in.
}

// Declares that this block is not reachable in the CFG.
// This instruction must be the last instruction in a
// block.
//
// Capability: Kernel
instruction! { OpUnreachable, 1, 1 }

// Declare that the content of the object pointed to was not defined before this instruction.
// If Operand 1 has a non-void type, Operand 2 must be 0, otherwise Operand 2 is the
// amount of memory whose lifetime is starting.
instruction! { OpLifetimeStart, 3, 3,
    operand1: Id,
    operand2: LiteralNumber
}

instruction! { OpLifetimeStop, 3, 3,
// Declare that the content of the object pointed to is dead after this instruction. If
// Operand 1 has a non-void type, Operand 2 must be 0, otherwise Operand 2 is the
// amount of memory whose life-time is ending.
    operand1: Id,
    operand2: LiteralNumber
}



///---------------------------
///3.27.17 Atomic Instructions
///---------------------------

instruction! { OpAtomicLoad, 6, 6,
// Atomically load through Pointer using the given Semantics. All subparts of the value that is loaded will be
// read atomically with respect to all other atomic accesses to it within Scope.
//
// Result Type must be the same type as the type pointed to by Pointer.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift
}

instruction! { OpAtomicStore, 5, 5,
// Atomically store through Pointer using the given Semantics. All subparts of Value will be written
// atomically with respect to all other atomic accesses to it within Scope.
//
// The type pointed to by Pointer must be the same type as the type of Value.
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicExchange, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value from copying Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicCompareExchange, 8, 8,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by selecting Value if Original Value equals Comparator or selecting Original Value otherwise, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id,
    comparator: Id
}

instruction! { OpAtomicCompareExchangeWeak, 8, 8,
// Attempts to do the following :
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by selecting Value if Original Value equals Comparator or selecting Original Value otherwise, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type. This type must also match the type
// of Comparator.
//
// TBD. What is the result if the operation fails ?
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id,
    comparator: Id
}

instruction! { OpAtomicIIncrement, 6, 6,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value through integer addition of 1 to Original Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type must be the same type as the type pointed to by Pointer.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift
}

instruction! { OpAtomicIDecrement, 6, 6,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location:
// 1 load through Pointer to get an Original Value,
// 2 get a New Value through integer subtraction of 1 from Original Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type must be the same type as the type pointed to by Pointer.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift
}

instruction! { OpAtomicIAdd, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by integer addition of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicISub, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by integer subtraction of Value from Original Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicUMin, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by finding the smallest unsigned integer of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicUMax, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by finding the largest unsigned integer of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicAnd, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by the bitwise AND of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicOr, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by the bitwise OR of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicXor, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by the bitwise exclusive OR of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicIMin, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by finding the smallest signed integer of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}

instruction! { OpAtomicIMax, 7, 7,
// Perform the following steps atomically with respect to any other atomic accesses within Scope to the same location :
// 1 load through Pointer to get an Original Value,
// 2 get a New Value by finding the largest signed integer of Original Value and Value, and
// 3 store the New Value back through Pointer.
//
// The instruction's result is the Original Value.
//
// Result Type, the type of Value, and the type pointed to by Pointer must all be same type.
    result_type: Id,
    result: Id,
    pointer: Id,
    scope: Scope,
    semantics: MemorySemanticsShift,
    value: Id
}



///------------------------------
///3.27.18 Primitive Instructions
///------------------------------




// Emits the current values of all output
// variables to the current output primitive.
// After execution, the values of all output
// variables are undefined.
// This instruction can only be used when
// only one stream is present.
//
// Capability: Geom
instruction! { OpEmitVertex, 1, 1  }

// Finish the current primitive
// and start a new one. No
// vertex is emitted.
// This instruction can only
// be used when only one
// stream is present.
//
// Capability: Geom
instruction! { OpEndPrimitive, 1, 1  }

instruction! { OpEmitStreamVertex, 2, 2,
// Emits the current values of all output variables to
// the current output primitive. After execution, the
// values of all output variables are undefined.
//
// Stream must be an <id> of a constant instruction
// with a scalar integer type. It is the stream the
// primitive is on.
//
// This instruction can only be used when multiple
// streams are present.
//
// Capability: Geom
    stream: Id
}

instruction! { OpEndStreamPrimitive, 2, 2,
// Finish the current primitive and start a new one.
// No vertex is emitted.
//
// Stream must be an <id> of a constant instruction
// with a scalar integer type. It is the stream the
// primitive is on.
//
// This instruction can only be used when multiple
// streams are present.
//
// Capability: Geom
    stream: Id
}



///------------------------------
///3.27.19 Barrier Instructions
///------------------------------

instruction! { OpControlBarrier, 2, 2,
// Wait for other invocations of this module to reach this same point of execution.
//
// All invocations of this module within Scope must reach this point of execution before any will proceed beyond it.
//
// This instruction is only guaranteed to work correctly if placed strictly within dynamically uniform control flow within
// Scope. This ensures that if any invocation executes it, all invocations will execute it. If placed elsewhere, an invocation
// may stall indefinitely.
//
// It is only valid to use this instruction with TessellationControl, GLCompute, or Kernel execution models.
    scope: Scope
}

instruction! { OpMemoryBarrier, 3, 3,
// Control the order that memory accesses are observed.
//
// Ensures that memory accesses issued before this instruction will be observed before memory accesses issued after this
// instruction. This control is ensured only for memory accesses issued by this invocation and observed by another invocation
// executing within Scope.
//
// Semantics declares what kind of memory is being controlled and what kind of control to apply.
    scope: Scope,
    semantics: MemorySemanticsShift
}

///--------------------------
///3.27.20 Group Instructions
///--------------------------

instruction! { OpGroupAsyncCopy, 9, 9,
// Perform an asynchronous group copy of Num Elements elements from Source to Destination. The
// asynchronous copy is performed by all work-items in a group.
//
// Returns an event object that can be used by OpGroupWaitEvents to wait for the copy to finish.
//
// Event must be OpTypeEvent.
//
// Event can be used to associate the copy with a previous copy allowing an event to be shared by multiple
// copies. Otherwise Event should be a OpConstantNullObject.
//
// If Event argument is not OpConstantNullObject, the event object supplied in event argument will be returned.
//
// Scope must be theWorkgroup or Subgroup Execution Scope.
//
// Destination and Source should both be pointers to the same integer or floating point scalar or vector data type.
//
// Destination and Source pointer storage class can be eitherWorkgroupLocal or WorkgroupGlobal.
//
// When Destination pointer storage class isWorkgroupLocal, the Source pointer storage class must be
// WorkgroupGlobal. In this case Stride defines the stride in elements when reading from Source pointer.
//
// When Destination pointer storage class isWorkgroupGlobal, the Source pointer storage class must be
// WorkgroupLocal. In this case Stride defines the stride in elements when writing each element to
// Destination pointer.
//
// Stride and NumElemens must be a 32 bit OpTypeInt when the Addressing Model is Physical32 and 64 bit
// OpTypeInt when the Addressing Model is Physical64.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    scope: Scope,
    destination: Id,
    source: Id,
    numElements: Id,
    stride: Id,
    event: Id
}

instruction! { OpGroupWaitEvents, 6, 6,
// Wait for events generated by OpGroupAsyncCopy operations to complete. The event objects pointed
// by Events List will be released after the wait is performed.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    num_events: Id,          // Must be a 32 bits wide OpTypeInt.
    events_list: Id        // Must be a pointer to OpTypeEvent.
}

instruction! { OpGroupAll, 5, 5,
// Evaluates a predicate for all work-items in the group,and returns true if predicate evaluates to
// true for all work-items in the group, otherwise returns false.
//
// Capability: Kernel
    result_type: Id,         // Must be of OpTypeBool.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    predicate: Id         // Must be of OpTypeBool.
}

instruction! { OpGroupAny, 5, 5,
// Evaluates a predicate for all work-items in the group,and returns true if predicate evaluates to
// true for any work-item in the group, otherwise returns false.
//
// Capability: Kernel
    result_type: Id,         // Must be of OpTypeBool.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    predicate: Id         // Must be of OpTypeBool.
}

instruction! { OpGroupBroadcast, 6, 6,
// Broadcast a value for workitem identified by the local id to all work-items in the group.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 or 64 bits wise OpTypeInt or a 16, 32 or 64 OpTypeFloat
                                            // floating-point scalar datatype.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    value: Id,              // Must be a 32 or 64 bits wise OpTypeInt or a 16, 32 or 64 OpTypeFloat
                                            // floating-point scalar datatype.
    local_id: Id             // Must be an integer datatype. It can be a scalar, or a vector with 2 components or a vector
                                            // with 3 components. LocalId must be the same for all work-items in the group.
}

instruction! { OpGroupIAdd, 6, 6,
// An integer add group operation specified for all values of X specified by work-items in the group.
//
// The identity I is 0.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 or 64 bits wide OpTypeInt data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 32 or 64 bits wide OpTypeInt data type.
}

instruction! { OpGroupFAdd, 6, 6,
// A floating-point add group operation specified for all values of X specified by work-items in the
// group.
//
// The identity I is 0.
//
// Capability: Kernel
    result_type: Id,         // Must be a 16, 32 or 64 bits wide OpTypeFloat data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 16, 32 or 64 bits wide OpTypeFloat data type.
}

instruction! { OpGroupFMin, 6, 6,
// A floating-point minimum group operation specified for all values of X specified by work-items in
// the group.
//
// The identity I is + INF.
//
// Capability: Kernel
    result_type: Id,         // Must be a 16, 32 or 64 bits wide OpTypeFloat data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 16, 32 or 64 bits wide OpTypeFloat data type.
}

instruction! { OpGroupUMin, 6, 6,
// An unsigned integer minimum group operation specified for all values of X specified by work-items
// in the group.
//
// The identity I is UINT_MAX when X is 32 bits wide and ULONG_MAX when X is 64 bits wide.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 or 64 bits wide OpTypeInt data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 32 or 64 bits wide OpTypeInt data type.
}

instruction! { OpGroupSMin, 6, 6,
// A signed integer minimum group operation specified for all values of X specified by work-items in
// the group.
//
// The identity I is INT_MAX when X is 32 bits wide and LONG_MAX when X is 64 bits wide.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 or 64 bits wide OpTypeInt data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 32 or 64 bits wide OpTypeInt data type.
}

instruction! { OpGroupFMax, 6, 6,
// A floating-point maximum group operation specified for all values of X specified by work-items in
// the group.
//
// The identity I is-INF.
//
// Capability: Kernel
    result_type: Id,         // Must be a 16, 32 or 64 bits wide OpTypeFloat data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 16, 32 or 64 bits wide OpTypeFloat data type.
}

instruction! { OpGroupUMax, 6, 6,
// An unsigned integer maximum group operation specified for all values of X specified by work-items
// in the group.
//
// The identity I is 0.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 or 64 bits wide OpTypeInt data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 32 or 64 bits wide OpTypeInt data type.
}

instruction! { OpGroupSMax, 6, 6,
// A signed integer maximum group operation specified for all values of X specified by work-items in
// the group.
//
// The identity I is INT_MIN when X is 32 bits wide and LONG_MIN when X is 64 bits wide.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 or 64 bits wide OpTypeInt data type.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    operation: GroupOperation,
    x: Id                   // Must be a 32 or 64 bits wide OpTypeInt data type.
}



///----------------------------------------
///3.27.21 Device-Side Enqueue Instructions
///----------------------------------------



instruction! { OpEnqueueMarker, 7, 7,
// Enqueue a marker command to to the queue object specified by q. The marker command waits for a list
// of events to complete, or if the list is empty it waits for all previously enqueued commands in q to
// complete before the marker completes.
//
// These are the possible return values:
// A successfull enqueue is indicated by the integer value 0
// A failed enqueue is indicated by the negative integer value -101
//
// When running the clCompileProgram or clBuildProgram with -g flag, the following errors may be returned instead of the negative integer value -101:
// - When q is an invalid queue object, the negative integer value -102 is returned.
// - When Wait Events is null and Num Events > 0, or if Wait Events is not null and Num Events is 0, or if event objects in Wait Events are not valid events, the negative integer value -57 is returned.
// - When the queue object q is full, the negative integer value -161 is returned.
// - When Ret Event is not a null object and an event could not be allocated, the negative integer value -100 is returned.
// - When there is a failure to queue Invoke in the queue q because of insufficient resources needed to execute the kernel, the negative integer value -5 is returned.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 bit OpTypeInt.
    result: Id,
    q: Id,                  //
    num_events: Id,          // specifies the number of event objects in the wait list pointed Wait Events and must be 32 bit OpTypeInt treated as unsigned integer.
    wait_events: Id,         // specifies the list of wait event objects and must be a OpTypePointer to OpTypeDeviceEvent.
    ret_events: Id           // OpTypePointer to OpTypeDeviceEvent which gets implictly retained by this instruction. must be a OpTypePointer to OpTypeDeviceEvent. If Ret Event is set to null this instruction becomes a no-op.
}

// Enqueue the the function specified by Invoke and the NDRange specified by ND Range for execution to the queue object specified by q.
//
// These are the possible return values:
// A successfull enqueue is indicated by the integer value 0
// A failed enqueue is indicated by the negative integer value -101
//
// When running the clCompileProgram or clBuildProgram with -g flag, the following errors may be returned instead of the negative value -101:
// - When q is an invalid queue object, the negative integer value -102 is returned.
// - When ND Range is an invalid descriptor or if the program was compiled with -cl-uniform-work-group-size and the local work size is specified in ndrange but the global work size specified in ND Range is not a multiple of the local work size, the negative integer value -160 is returned.
// - When Wait Events is null and Num Events > 0, or if Wait Events is not null and Num Events is 0, or if event objects in Wait Events are not valid events, the negative integer value -57 is returned.
// - When the queue object q is full, the negative integer value -161 is returned.
// - When one of the operands Local Size is 0, the negative integer value -51 is returned.
// - When Ret Event is not a null object and an event could not be allocated, the negative integer value -100 is returned.
// - When there is a failure to queue Invoke in the queue q because of insufficient resources needed to execute the kernel, the negative integer value -5 is returned.
//
// Capability: Kernel
instruction! { OpEnqueueKernel, 13, 65535,
    result_type: Id,         // Must be a 32 bit OpTypeInt.
    result: Id,
    q: Id,
    flags: KernelEnqueueFlags,
    nd_range: Id,            // Must be a OpTypeStruct created by OpBuildNDRange.
    num__events: Id,          // specifies the number of event objects in the wait list pointed Wait Events and must be 32 bit OpTypeInt treated as unsigned integer.
    wait__events: Id,         // specifies the list of wait event objects and must be a OpTypePointer to OpTypeDeviceEvent.
    ret__events: Id,          // OpTypePointer to OpTypeDeviceEvent which gets implictly retained by this instruction. must be a OpTypePointer to OpTypeDeviceEvent.
    invoke: Id,             // Must be a OpTypeFunction with the following signature:
                                            // - Result Type must be OpTypeVoid.
                                            // - The first parameter must be OpTypePointer to 8 bits OpTypeInt.
                                            // - Optional list of parameters that must be OpTypePointer with WorkgroupLocal storage class.
    param: Id,              // the first parameter of the function specified by Invoke and must be OpTypePointer to 8 bit OpTypeInt.
    param_size: Id,          // the size in bytes of the memory pointed by Param and must be a 32 bit OpTypeInt treated as unsigned int.
    param_align: Id,         // the alignment of Param.
    local_size: UnsizedArray<Id>       // (optional) list of 32 bit OpTypeInt values which are treated as unsigned integers. Every Local Size specifies the size in bytes of the OpTypePointer with WorkgroupLocal of Invoke. The number of Local Size operands must match the signature of Invoke OpTypeFunction
}

// Returns the number of subgroups in each workgroup of the dispatch (except for the last in cases where the global size does not divide cleanly into work-groups) given the combination of the passed NDRange descriptor specified by ND Range and the function specified by Invoke.
//
// Capability: Kernel
instruction! { OpGetKernelNDrangeSubGroupCount, 5, 5,
    result_type: Id,         // Must be a 32 bit OpTypeInt.
    result: Id,
    nd_range: Id,            // Must be a OpTypeStruct created by OpBuildNDRange.
    invoke: Id             // Must be a OpTypeFunction with the following signature:
                                            // - Result Type must be OpTypeVoid.
                                            // - The first parameter must be OpTypePointer to 8 bits OpTypeInt.
                                            // - Optional list of parameters that must be OpTypePointer with WorkgroupLocal storage class.
}

instruction! { OpGetKernelNDrangeMaxSubGroupSize, 5, 5,
// Returns the maximum sub-group size for the function specified by Invoke and the NDRange specified by ND Range.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 bit OpTypeInt.
    result: Id,
    nd_range: Id,            // Must be a OpTypeStruct created by OpBuildNDRange.
    invoke: Id             // Must be a OpTypeFunction with the following signature:
                                            // - Result Type must be OpTypeVoid.
                                            // - The first parameter must be OpTypePointer to 8 bits OpTypeInt.
                                            // - Optional list of parameters that must be OpTypePointer with WorkgroupLocal storage class.
}

instruction! { OpGetKernelWorkGroupSize, 4, 4,
// Returns the maximum work-group size that can be used to execute the function specified by Invoke on the device.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 bit OpTypeInt.
    result: Id,
    invoke: Id             // Must be a OpTypeFunction with the following signature:
                                            // - Result Type must be OpTypeVoid.
                                            // - The first parameter must be OpTypePointer to 8 bits OpTypeInt.
                                            // - Optional list of parameters that must be OpTypePointer with WorkgroupLocal storage class.
}

instruction! { OpGetKernelPreferredWorkGroupSizeMultiple, 4, 4,
// Returns the preferred multiple of work-group size for the function specified by Invoke.
// This is a performance hint. Specifying a work-group size that is not a multiple of the
// value returned by this query as the value of the local work size will not fail to enqueue
// Invoke for execution unless the work-group size specified is larger than the device
// maximum.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32 bit OpTypeInt.
    result: Id,
    invoke: Id             // Must be a OpTypeFunction with the following signature:
                                            // - Result Type must be OpTypeVoid.
                                            // - The first parameter must be OpTypePointer to 8 bits OpTypeInt.
                                            // - Optional list of parameters that must be OpTypePointer with WorkgroupLocal storage class.
}

instruction! { OpRetainEvent, 2, 2,
// Increments the reference count of the
// event object specified by event.
//
// Capability: Kernel
    event: Id               // Must be an event that was
                                            // produced by OpEnqueueKernel,
                                            // OpEnqueueMarker or
                                            // OpCreateUserEvent.
}

instruction! { OpReleaseEvent, 2, 2,
// Decrements the reference count of the event
// object specified by event. The event object is
// deleted once the event reference count is zero,
// the specific command identified by this event has
// completed (or terminated) and there are no
// commands in any device command queue that
// require a wait for this event to complete.
//
// Capability: Kernel
    event: Id               // Must be an event that was produced by
                                            // OpEnqueueKernel, OpEnqueueMarker or
                                            // OpCreateUserEvent.
}

instruction! { OpCreateUserEvent, 3, 3,
// Create a user event. The execution status
// of the created event is set to a value of 2
// (CL_SUBMITTED).
//
// Capability: Kernel
    result_type: Id,         // Must be OpTypeDeviceEvent.
    result: Id
}

instruction! { OpIsValidEvent, 4, 4,
// Returns true if the event specified by event is a valid event,
// otherwise returns false.
//
// Capability: Kernel
    result_type: Id,         // Must be a OpTypeBool.
    result: Id,
    event: Id             // Must be a OpTypeDeviceEvent
}

instruction! { OpSetUserEventStatus, 3, 3,
// Sets the execution status of a user event specified by event. status can be
// either 0 (CL_COMPLETE) to indicate that this kernel and all its child
// kernels finished execution successfully, or a negative integer value indicating
// an error.
//
// Capability: Kernel
    event: Id,              // Must be a OpTypeDeviceEvent that was produced by
                                            // OpCreateUserEvent.
    status: Id              // Must be a 32-bit OpTypeInt treated as a signed integer.
}

instruction! { OpCaptureEventProfilingInfo, 4, 4,
// Captures the profiling information specified by info for the command associated with the
// event specified by event in the memory pointed by value. The profiling information will
// be available in value once the command identified by event has completed.
//
// When info is CmdExecTime value must be a OpTypePointer with WorkgroupGlobal
// storage class, to two 64-bit OpTypeInt values. The first 64-bit value describes the elapsed
// time CL_PROFILING_COMMAND_END-CL_PROFLING_COMMAND_START for
// the command identified by event in nanoseconds. The second 64-bit value describes the
// elapsed time CL_PROFILING_COMMAND_COMPLETE-CL_PROFILING_COMAMND_START
// for the command identified by event in nanoseconds.
//
// Note: The behavior of of this instruction is undefined when called multiple times for the
// same event.
//
// Capability: Kernel
    event: Id,              // Must be a OpTypeDeviceEvent that was produced by OpEnqueueKernel or
                                            // OpEnqueueMarker.
    info: KernelProfilingInfoShift,               //
    status: Id            //
}

instruction! { OpGetDefaultQueue, 3, 3,
// Returns the default device queue. If a default device queue
// has not been created, null queue object is returned using the
// OpConstantNullObject instruction.
//
// Capability: Kernel
    result_type: Id,         // Must be a OpTypeQueue.
    result: Id
}

instruction! { OpBuildNDRange, 6, 6,
// Given the global work size specified by GlobalWorkSize, local work size specified by LocalWorkSize
// and global work offset specified by GlobalWorkOffset, builds a 1D, 2D or 3D ND-range descriptor
// structure.
//
// GlobalWorkSize, LocalWorkSize and GlobalWorkOffset must be a scalar or an array with 2 or 3
// components. Where the type of each element in the array is 32 bit OpTypeInt when the Addressing
// Model is Physical32 or 64 bit OpTypeInt when the Addressing Model is Physical64.
//
// Result Type is the descriptor and must be a OpTypeStruct with the following ordered list of members,
// starting from the first to last:
// - 32 bit OpTypeInt that specifies the number of dimensions used to specify the global work-items and
//   work-items in the work-group.
// - OpTypeArray with 3 elements, where each element is 32 bit OpTypeInt when the Addressing
//   Model is Physical32 and 64 bit OpTypeInt when the Addressing Model is Physical64. This
//   member is an array of per-dimension unsigned values that describe the offset used to calculate the
//   global Id of a work-item.
// - OpTypeArray with 3 elements, where each element is 32 bit OpTypeInt when the Addressing
//   Model is Physical32 and 64 bit OpTypeInt when the Addressing Model is Physical64. This
//   member is an array of per-dimension unsigned values that describe the number of global work-items
//   in the dimensions that will execute the kernel function.
// - OpTypeArray with 3 elements, where each element is 32 bit OpTypeInt when the Addressing
//   Model is Physical32 and 64 bit OpTypeInt when the Addressing Model is Physical64. This
//   member is an array of an array of per-dimension unsigned values that describe the number of
//   work-items that make up a work-group.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    global_work_size: Id,
    local_work_size: Id,
    global_work_offset: Id
}



///----------------------------------------
///3.27.22 Pipe Instructions
///----------------------------------------



instruction! { OpReadPipe, 5, 5,
// Read a packet from the pipe object specified by p into ptr. Returns 0 if the operation is
// successfull and a negative value if the pipe is empty.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    p: Id,                  // Must be a OpTypePipe with ReadOnly Access Qualifier.
    ptr: Id               // Must be a OpTypePointer with the same data type as p and a Generic storage class.
}

instruction! { OpWritePipe, 5, 5,
// Write a packet from ptr to the pipe object specified by p. Returns 0 if the operation is successfull
// and a negative value if the pipe is full.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32-bits OpTypeInt.
    result: Id,
    p: Id,                  // Must be a OpTypePipe with WriteOnly Access Qualifier.
    ptr: Id               // Must be a OpTypePointer with the same data type as p and a Generic storage class.
}

instruction! { OpReservedReadPipe, 7, 7,
// Read a packet from the reserved area specified by reserve_id and index of the pipe object specified by p
// into ptr. The reserved pipe entries are referred to by indices that go from 0 ... num_packets-1.
// Returns 0 if the operation is successfull and a negative value otherwise.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32-bits OpTypeInt.
    result: Id,
    p: Id,                  // Must be a OpTypePipe with ReadOnly Access Qualifier.
    reserve_id: Id,         // Must be a OpTypeReserveId.
    index: Id,              // Must be a 32-bits OpTypeInt which is treated as unsigned value.
    ptr: Id               // Must be a OpTypePointer with the same data type as p and a Generic storage class.
}

instruction! { OpReservedWritePipe, 7, 7,
// Write a packet from ptr into the reserved area specified by reserve_id and index of the pipe object
// specified by p. The reserved pipe entries are referred to by indices that go from 0 ... num_packets -1.
// Returns 0 if the operation is successfull and a negative value otherwise.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32-bits OpTypeInt.
    result: Id,
    p: Id,                  // Must be a OpTypePipe with WriteOnly Access Qualifier.
    reserve_id: Id,         // Must be a OpTypeReserveId.
    index: Id,              // Must be a 32-bits OpTypeInt which is treated as unsigned value.
    ptr: Id               // Must be a OpTypePointer with the same data type as p and a Generic storage class.
}

instruction! { OpReserveReadPipePackets, 5, 5,
// Reserve num_packets entries for reading from the pipe object
// specified by p. Returns a valid reservation Id if the reservation is
// successful.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    p: Id,
    num_packets: Id
}

instruction! { OpReserveWritePipePackets, 5, 5,
// Reserve num_packets entries for writing to the pipe object specified
// by p. Returns a valid reservation Id if the reservation is successful.
//
// Capability: Kernel
    result_type: Id,
    result: Id,
    p: Id,
    num_packets: Id
}

instruction! { OpCommitReadPipe, 3, 3,
// Indicates that all reads to num_packets associated with the reservation
// specified by reserve_id and the pipe object specified by p are completed.
//
// Capability: Kernel
    p: Id,                  // Must be a OpTypePipe with ReadOnly Access Qualifier.
    reserve_id: Id         // Must be a OpTypeReserveId.
}

instruction! { OpCommitWritePipe, 3, 3,
// Indicates that all writes to num_packets associated with the reservation
// specified by reserve_id and the pipe object specified by p are completed.
//
// Capability: Kernel
    p: Id,                  // Must be a OpTypePipe with WriteOnly Access Qualifier.
    reserve_id: Id         // Must be a OpTypeReserveId.
}

instruction! { OpIsValidReserveId, 4, 4,
// Return true if reserve_id is a valid reservation Id and false
// otherwise.
//
// Capability: Kernel
    result_type: Id,         // Must be a OpTypeBool.
    result: Id,
    reserve_id: Id         // Must be a OpTypeReserveId.
}

instruction! { OpGetNumPipePackets, 4, 4,
// Returns the number of available entries in the pipe object specified by p. The number of
// available entries in a pipe is a dynamic value. The value returned should be considered
// immediately stale.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32-bits OpTypeInt which should be treated as unsigned value.
    result: Id,
    p: Id                 // Must be a OpTypePipe with ReadOnly or WriteOnly Access Qualifier.
}

instruction! { OpGetMaxPipePackets, 4, 4,
// Returns the maximum number of packets specified when the pipe object specified by p
// was created.
//
// Capability: Kernel
    result_type: Id,         // Must be a 32-bits OpTypeInt which should be treated as unsigned value.
    result: Id,
    p: Id                 // Must be a OpTypePipe with ReadOnly or WriteOnly Access Qualifier.
}

instruction! { OpGroupReserveReadPipePackets, 6, 6,
// Reserve num_packets entries for reading from the pipe object specified by p at group level. Returns a
// valid reservation Id if the reservation is successful.
//
// The reserved pipe entries are referred to by indices that go from 0 ... num_packets-1.
//
// Capability: Kernel
    result_type: Id,         // Must be a OpTypeReserveId.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    p: Id,                  // Must be a OpTypePipe with ReadOnly Access Qualifier.
    num_packets: Id // Must be a 32-bits OpTypeInt which is treated as unsigned value.
}

instruction! { OpGroupReserveWritePipePackets, 6, 6,
// Reserve num_packets entries for writing to the pipe object specified by p at group level. Returns a
// valid reservation Id if the reservation is successful.
//
// The reserved pipe entries are referred to by indices that go from 0 ... num_packets-1.
//
// Capability: Kernel
    result_type: Id,         // Must be a OpTypeReserveId.
    result: Id,
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    p: Id,                  // Must be a OpTypePipe with WriteOnly Access Qualifier.
    num_packets: Id // Must be a 32-bits OpTypeInt which is treated as unsigned value.
}

instruction! { OpGroupCommitReadPipe, 4, 4,
// A group level indication that all reads to num_packets associated with the reservation
// specified by reserve_id to the pipe object specified by p are completed.
//
// Capability: Kernel
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    p: Id,                  // Must be a OpTypePipe with ReadOnly Access Qualifier.
    reserve_id: Id         // Must be a OpTypeReserveId.
}

instruction! { OpGroupCommitWritePipe, 4, 4,
// A group level indication that all writes to num_packets associated with the reservation
// specified by reserve_id to the pipe object specified by p are completed.
//
// Capability: Kernel
    scope: Scope,              // Must be theWorkgroup or Subgroup Execution Scope.
    p: Id,                  // Must be a OpTypePipe with WriteOnly Access Qualifier.
    reserve_id: Id         // Must be a OpTypeReserveId.
}
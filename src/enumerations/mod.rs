#![allow(dead_code)]

/*
** Copyright (c) 2014-2015 The Khronos Group Inc.
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and/or associated documentation files (the "Materials"),
** to deal in the Materials without restriction, including without limitation
** the rights to use, copy, modify, merge, publish, distribute, sublicense,
** and/or sell copies of the Materials, and to permit persons to whom the
** Materials are furnished to do so, subject to the following conditions:
**
** The above copyright notice and this permission notice shall be included in
** all copies or substantial portions of the Materials.
**
** MODIFICATIONS TO THIS FILE MAY MEAN IT NO LONGER ACCURATELY REFLECTS KHRONOS
** STANDARDS. THE UNMODIFIED, NORMATIVE VERSIONS OF KHRONOS SPECIFICATIONS AND
** HEADER INFORMATION ARE LOCATED AT https://www.khronos.org/registry/
**
** THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
** OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
** THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
** LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
** FROM,OUT OF OR IN CONNECTION WITH THE MATERIALS OR THE USE OR OTHER DEALINGS
** IN THE MATERIALS.
*/

pub mod glsl_std_450;

const VERSION: u32 = 100;
const REVISION: u32 = 2;

#[derive(Debug)]
pub enum SourceLanguage {
    SourceLanguageUnknown = 0,
    SourceLanguageESSL = 1,
    SourceLanguageGLSL = 2,
    SourceLanguageOpenCL_C = 3,
    SourceLanguageOpenCL_CPP = 4
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ExecutionModel {
    Vertex = 0,
    TessellationControl = 1,
    TessellationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    GLCompute = 5,
    Kernel = 6,
}

#[derive(Debug)]
#[repr(u32)]
pub enum AddressingModel {
    AddressingModelLogical = 0,
    AddressingModelPhysical32 = 1,
    AddressingModelPhysical64 = 2,
}

#[derive(Debug)]
#[repr(u32)]
pub enum MemoryModel {
    MemoryModelSimple = 0,
    MemoryModelGLSL450 = 1,
    MemoryModelOpenCL = 2,
}

#[derive(Debug)]
pub enum ExecutionMode {
    Invocations = 0,
    SpacingEqual = 1,
    SpacingFractionalEven = 2,
    SpacingFractionalOdd = 3,
    VertexOrderCw = 4,
    VertexOrderCcw = 5,
    PixelCenterInteger = 6,
    OriginUpperLeft = 7,
    OriginLowerLeft = 8,
    EarlyFragmentTests = 9,
    PointMode = 10,
    Xfb = 11,
    DepthReplacing = 12,
    DepthAny = 13,
    DepthGreater = 14,
    DepthLess = 15,
    DepthUnchanged = 16,
    LocalSize = 17,
    LocalSizeHint = 18,
    InputPoints = 19,
    InputLines = 20,
    InputLinesAdjacency = 21,
    Triangles = 22,
    InputTrianglesAdjacency = 23,
    Quads = 24,
    Isolines = 25,
    OutputVertices = 26,
    OutputPoints = 27,
    OutputLineStrip = 28,
    OutputTriangleStrip = 29,
    VecTypeHint = 30,
    ContractionOff = 31,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StorageClass {
    UniformConstant = 0,
    Input = 1,
    Uniform = 2,
    Output = 3,
    Workgroup = 4,
    CrossWorkgroup = 5,
    Private = 6,
    Function = 7,
    Generic = 8,
    AtomicCounter = 10,
    Image = 11,
}

#[derive(Debug)]
pub enum Dim {
    Dim1D = 0,
    Dim2D = 1,
    Dim3D = 2,
    DimCube = 3,
    DimRect = 4,
    DimBuffer = 5,
    DimSubpassData = 6,
}

#[derive(Debug)]
pub enum SamplerAddressingMode {
    SamplerAddressingModeNone = 0,
    SamplerAddressingModeClampToEdge = 1,
    SamplerAddressingModeClamp = 2,
    SamplerAddressingModeRepeat = 3,
    SamplerAddressingModeRepeatMirrored = 4,
}

#[derive(Debug)]
pub enum SamplerFilterMode {
    SamplerFilterModeNearest = 0,
    SamplerFilterModeLinear = 1,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ImageFormat {
    ImageFormatUnknown = 0,
    ImageFormatRgba32f = 1,
    ImageFormatRgba16f = 2,
    ImageFormatR32f = 3,
    ImageFormatRgba8 = 4,
    ImageFormatRgba8Snorm = 5,
    ImageFormatRg32f = 6,
    ImageFormatRg16f = 7,
    ImageFormatR11fG11fB10f = 8,
    ImageFormatR16f = 9,
    ImageFormatRgba16 = 10,
    ImageFormatRgb10A2 = 11,
    ImageFormatRg16 = 12,
    ImageFormatRg8 = 13,
    ImageFormatR16 = 14,
    ImageFormatR8 = 15,
    ImageFormatRgba16Snorm = 16,
    ImageFormatRg16Snorm = 17,
    ImageFormatRg8Snorm = 18,
    ImageFormatR16Snorm = 19,
    ImageFormatR8Snorm = 20,
    ImageFormatRgba32i = 21,
    ImageFormatRgba16i = 22,
    ImageFormatRgba8i = 23,
    ImageFormatR32i = 24,
    ImageFormatRg32i = 25,
    ImageFormatRg16i = 26,
    ImageFormatRg8i = 27,
    ImageFormatR16i = 28,
    ImageFormatR8i = 29,
    ImageFormatRgba32ui = 30,
    ImageFormatRgba16ui = 31,
    ImageFormatRgba8ui = 32,
    ImageFormatR32ui = 33,
    ImageFormatRgb10a2ui = 34,
    ImageFormatRg32ui = 35,
    ImageFormatRg16ui = 36,
    ImageFormatRg8ui = 37,
    ImageFormatR16ui = 38,
    ImageFormatR8ui = 39,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ImageChannelOrder {
    ImageChannelOrderR = 0,
    ImageChannelOrderA = 1,
    ImageChannelOrderRG = 2,
    ImageChannelOrderRA = 3,
    ImageChannelOrderRGB = 4,
    ImageChannelOrderRGBA = 5,
    ImageChannelOrderBGRA = 6,
    ImageChannelOrderARGB = 7,
    ImageChannelOrderIntensity = 8,
    ImageChannelOrderLuminance = 9,
    ImageChannelOrderRx = 10,
    ImageChannelOrderRGx = 11,
    ImageChannelOrderRGBx = 12,
    ImageChannelOrderDepth = 13,
    ImageChannelOrderDepthStencil = 14,
    ImageChannelOrdersRGB = 15,
    ImageChannelOrdersRGBx = 16,
    ImageChannelOrdersRGBA = 17,
    ImageChannelOrdersBGRA = 18,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ImageChannelDataType {
    ImageChannelDataTypeSnormInt8 = 0,
    ImageChannelDataTypeSnormInt16 = 1,
    ImageChannelDataTypeUnormInt8 = 2,
    ImageChannelDataTypeUnormInt16 = 3,
    ImageChannelDataTypeUnormShort565 = 4,
    ImageChannelDataTypeUnormShort555 = 5,
    ImageChannelDataTypeUnormInt101010 = 6,
    ImageChannelDataTypeSignedInt8 = 7,
    ImageChannelDataTypeSignedInt16 = 8,
    ImageChannelDataTypeSignedInt32 = 9,
    ImageChannelDataTypeUnsignedInt8 = 10,
    ImageChannelDataTypeUnsignedInt16 = 11,
    ImageChannelDataTypeUnsignedInt32 = 12,
    ImageChannelDataTypeHalfFloat = 13,
    ImageChannelDataTypeFloat = 14,
    ImageChannelDataTypeUnormInt24 = 15,
    ImageChannelDataTypeUnormInt101010_2 = 16,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ImageOperandsShift {
    ImageOperandsBiasShift = 0,
    ImageOperandsLodShift = 1,
    ImageOperandsGradShift = 2,
    ImageOperandsConstOffsetShift = 3,
    ImageOperandsOffsetShift = 4,
    ImageOperandsConstOffsetsShift = 5,
    ImageOperandsSampleShift = 6,
    ImageOperandsMinLodShift = 7,
}

#[derive(Debug)]
pub enum ImageOperandsMask {
    ImageOperandsMaskNone = 0,
    ImageOperandsBiasMask = 0x00000001,
    ImageOperandsLodMask = 0x00000002,
    ImageOperandsGradMask = 0x00000004,
    ImageOperandsConstOffsetMask = 0x00000008,
    ImageOperandsOffsetMask = 0x00000010,
    ImageOperandsConstOffsetsMask = 0x00000020,
    ImageOperandsSampleMask = 0x00000040,
    ImageOperandsMinLodMask = 0x00000080,
}

#[derive(Debug)]
pub enum FPFastMathModeShift {
    FPFastMathModeNotNaNShift = 0,
    FPFastMathModeNotInfShift = 1,
    FPFastMathModeNSZShift = 2,
    FPFastMathModeAllowRecipShift = 3,
    FPFastMathModeFastShift = 4,
}

#[derive(Debug)]
pub enum FPFastMathModeMask {
    FPFastMathModeMaskNone = 0,
    FPFastMathModeNotNaNMask = 0x00000001,
    FPFastMathModeNotInfMask = 0x00000002,
    FPFastMathModeNSZMask = 0x00000004,
    FPFastMathModeAllowRecipMask = 0x00000008,
    FPFastMathModeFastMask = 0x00000010,
}

#[derive(Debug)]
pub enum FPRoundingMode {
    FPRoundingModeRTE = 0,
    FPRoundingModeRTZ = 1,
    FPRoundingModeRTP = 2,
    FPRoundingModeRTN = 3,
}

#[derive(Debug)]
#[repr(u32)]
pub enum LinkageType {
    LinkageTypeExport = 0,
    LinkageTypeImport = 1,
}

#[derive(Debug)]
#[repr(u32)]
pub enum AccessQualifier {
    AccessQualifierReadOnly = 0,
    AccessQualifierWriteOnly = 1,
    AccessQualifierReadWrite = 2,
}

#[derive(Debug)]
pub enum FunctionParameterAttribute {
    FunctionParameterAttributeZext = 0,
    FunctionParameterAttributeSext = 1,
    FunctionParameterAttributeByVal = 2,
    FunctionParameterAttributeSret = 3,
    FunctionParameterAttributeNoAlias = 4,
    FunctionParameterAttributeNoCapture = 5,
    FunctionParameterAttributeNoWrite = 6,
    FunctionParameterAttributeNoReadWrite = 7,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Decoration {
    RelaxedPrecision = 0,
    SpecId = 1,
    Block = 2,
    BufferBlock = 3,
    RowMajor = 4,
    ColMajor = 5,
    ArrayStride = 6,
    MatrixStride = 7,
    GLSLShared = 8,
    GLSLPacked = 9,
    CPacked = 10,
    BuiltIn = 11,
    Smooth = 12,
    NoPerspective = 13,
    Flat = 14,
    Patch = 15,
    Centroid = 16,
    Sample = 17,
    Invariant = 18,
    Restrict = 19,
    Aliased = 20,
    Volatile = 21,
    Constant = 22,
    Coherent = 23,
    NonWritable = 24,
    NonReadable = 25,
    Uniform = 26,
    SaturatedConversion = 28,
    Stream = 29,
    Location = 30,
    Component = 31,
    Index = 32,
    Binding = 33,
    DescriptorSet = 34,
    Offset = 35,
    XfbBuffer = 36,
    XfbStride = 37,
    FuncParamAttr = 38,
    FPRoundingMode = 39,
    FPFastMathMode = 40,
    LinkageAttributes = 41,
    NoContraction = 42,
    InputAttachmentIndex = 43,
    Alignment = 44,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BuiltIn {
    Position = 0,
    PointSize = 1,
    ClipDistance = 3,
    CullDistance = 4,
    VertexId = 5,
    InstanceId = 6,
    PrimitiveId = 7,
    InvocationId = 8,
    Layer = 9,
    ViewportIndex = 10,
    TessLevelOuter = 11,
    TessLevelInner = 12,
    TessCoord = 13,
    PatchVertices = 14,
    FragCoord = 15,
    PointCoord = 16,
    FrontFacing = 17,
    SampleId = 18,
    SamplePosition = 19,
    SampleMask = 20,
    FragColor = 21,
    FragDepth = 22,
    HelperInvocation = 23,
    NumWorkgroups = 24,
    WorkgroupSize = 25,
    WorkgroupId = 26,
    LocalInvocationId = 27,
    GlobalInvocationId = 28,
    LocalInvocationIndex = 29,
    WorkDim = 30,
    GlobalSize = 31,
    EnqueuedWorkgroupSize = 32,
    GlobalOffset = 33,
    GlobalLinearId = 34,
    WorkgroupLinearId = 35,
    SubgroupSize = 36,
    SubgroupMaxSize = 37,
    NumSubgroups = 38,
    NumEnqueuedSubgroups = 39,
    SubgroupId = 40,
    SubgroupLocalInvocationId = 41,
    VertexIndex = 42,
    InstanceIndex = 43,
}

#[derive(Debug)]
pub enum SelectionControlShift {
    SelectionControlFlattenShift = 0,
    SelectionControlDontFlattenShift = 1,
}

#[derive(Debug)]
pub enum SelectionControlMask {
    SelectionControlMaskNone = 0,
    SelectionControlFlattenMask = 0x00000001,
    SelectionControlDontFlattenMask = 0x00000002,
}

#[derive(Debug)]
pub enum LoopControlShift {
    LoopControlUnrollShift = 0,
    LoopControlDontUnrollShift = 1,
}

#[derive(Debug)]
pub enum LoopControlMask {
    LoopControlMaskNone = 0,
    LoopControlUnrollMask = 0x00000001,
    LoopControlDontUnrollMask = 0x00000002,
}

#[derive(Debug)]
pub enum FunctionControlShift {
    FunctionControlInlineShift = 0,
    FunctionControlDontInlineShift = 1,
    FunctionControlPureShift = 2,
    FunctionControlConstShift = 3,
}

#[derive(Debug)]
pub enum FunctionControlMask {
    FunctionControlMaskNone = 0,
    FunctionControlInlineMask = 0x00000001,
    FunctionControlDontInlineMask = 0x00000002,
    FunctionControlPureMask = 0x00000004,
    FunctionControlConstMask = 0x00000008,
}

#[derive(Debug)]
pub enum MemorySemanticsShift {
    MemorySemanticsAcquireShift = 1,
    MemorySemanticsReleaseShift = 2,
    MemorySemanticsAcquireReleaseShift = 3,
    MemorySemanticsSequentiallyConsistentShift = 4,
    MemorySemanticsUniformMemoryShift = 6,
    MemorySemanticsSubgroupMemoryShift = 7,
    MemorySemanticsWorkgroupMemoryShift = 8,
    MemorySemanticsCrossWorkgroupMemoryShift = 9,
    MemorySemanticsAtomicCounterMemoryShift = 10,
    MemorySemanticsImageMemoryShift = 11,
}

#[derive(Debug)]
pub enum MemorySemanticsMask {
    MemorySemanticsMaskNone = 0,
    MemorySemanticsAcquireMask = 0x00000002,
    MemorySemanticsReleaseMask = 0x00000004,
    MemorySemanticsAcquireReleaseMask = 0x00000008,
    MemorySemanticsSequentiallyConsistentMask = 0x00000010,
    MemorySemanticsUniformMemoryMask = 0x00000040,
    MemorySemanticsSubgroupMemoryMask = 0x00000080,
    MemorySemanticsWorkgroupMemoryMask = 0x00000100,
    MemorySemanticsCrossWorkgroupMemoryMask = 0x00000200,
    MemorySemanticsAtomicCounterMemoryMask = 0x00000400,
    MemorySemanticsImageMemoryMask = 0x00000800,
}

#[derive(Debug)]
pub enum MemoryAccessShift {
    MemoryAccessVolatileShift = 0,
    MemoryAccessAlignedShift = 1,
    MemoryAccessNontemporalShift = 2,
}

#[derive(Debug)]
pub enum MemoryAccessMask {
    MemoryAccessMaskNone = 0,
    MemoryAccessVolatileMask = 0x00000001,
    MemoryAccessAlignedMask = 0x00000002,
    MemoryAccessNontemporalMask = 0x00000004,
}

#[derive(Debug)]
pub enum Scope {
    ScopeCrossDevice = 0,
    ScopeDevice = 1,
    ScopeWorkgroup = 2,
    ScopeSubgroup = 3,
    ScopeInvocation = 4,
}

#[derive(Debug)]
pub enum GroupOperation {
    GroupOperationReduce = 0,
    GroupOperationInclusiveScan = 1,
    GroupOperationExclusiveScan = 2,
}

#[derive(Debug)]
pub enum KernelEnqueueFlags {
    KernelEnqueueFlagsNoWait = 0,
    KernelEnqueueFlagsWaitKernel = 1,
    KernelEnqueueFlagsWaitWorkGroup = 2,
}

#[derive(Debug)]
pub enum KernelProfilingInfoShift {
    KernelProfilingInfoCmdExecTimeShift = 0,
}

#[derive(Debug)]
pub enum KernelProfilingInfoMask {
    KernelProfilingInfoMaskNone = 0,
    KernelProfilingInfoCmdExecTimeMask = 0x00000001,
}

#[derive(Debug)]
pub enum Capability {
    CapabilityMatrix = 0,
    CapabilityShader = 1,
    CapabilityGeometry = 2,
    CapabilityTessellation = 3,
    CapabilityAddresses = 4,
    CapabilityLinkage = 5,
    CapabilityKernel = 6,
    CapabilityVector16 = 7,
    CapabilityFloat16Buffer = 8,
    CapabilityFloat16 = 9,
    CapabilityFloat64 = 10,
    CapabilityInt64 = 11,
    CapabilityInt64Atomics = 12,
    CapabilityImageBasic = 13,
    CapabilityImageReadWrite = 14,
    CapabilityImageMipmap = 15,
    CapabilityImageSRGBWrite = 16,
    CapabilityPipes = 17,
    CapabilityGroups = 18,
    CapabilityDeviceEnqueue = 19,
    CapabilityLiteralSampler = 20,
    CapabilityAtomicStorage = 21,
    CapabilityInt16 = 22,
    CapabilityTessellationPointSize = 23,
    CapabilityGeometryPointSize = 24,
    CapabilityImageGatherExtended = 25,
    CapabilityStorageImageExtendedFormats = 26,
    CapabilityStorageImageMultisample = 27,
    CapabilityUniformBufferArrayDynamicIndexing = 28,
    CapabilitySampledImageArrayDynamicIndexing = 29,
    CapabilityStorageBufferArrayDynamicIndexing = 30,
    CapabilityStorageImageArrayDynamicIndexing = 31,
    CapabilityClipDistance = 32,
    CapabilityCullDistance = 33,
    CapabilityImageCubeArray = 34,
    CapabilitySampleRateShading = 35,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum Op {
    Nop = 0,
    Undef = 1,
    SourceContinued = 2,
    Source = 3,
    SourceExtension = 4,
    Name = 5,
    MemberName = 6,
    String = 7,
    Line = 8,
    Extension = 10,
    ExtInstImport = 11,
    ExtInst = 12,
    MemoryModel = 14,
    EntryPoint = 15,
    ExecutionMode = 16,
    Capability = 17,
    TypeVoid = 19,
    TypeBool = 20,
    TypeInt = 21,
    TypeFloat = 22,
    TypeVector = 23,
    TypeMatrix = 24,
    TypeImage = 25,
    TypeSampler = 26,
    TypeSampledImage = 27,
    TypeArray = 28,
    TypeRuntimeArray = 29,
    TypeStruct = 30,
    TypeOpaque = 31,
    TypePointer = 32,
    TypeFunction = 33,
    TypeEvent = 34,
    TypeDeviceEvent = 35,
    TypeReserveId = 36,
    TypeQueue = 37,
    TypePipe = 38,
    TypeForwardPointer = 39,
    ConstantTrue = 41,
    ConstantFalse = 42,
    Constant = 43,
    ConstantComposite = 44,
    ConstantSampler = 45,
    ConstantNull = 46,
    SpecConstantTrue = 48,
    SpecConstantFalse = 49,
    SpecConstant = 50,
    SpecConstantComposite = 51,
    SpecConstantOp = 52,
    Function = 54,
    FunctionParameter = 55,
    FunctionEnd = 56,
    FunctionCall = 57,
    Variable = 59,
    ImageTexelPointer = 60,
    Load = 61,
    Store = 62,
    CopyMemory = 63,
    CopyMemorySized = 64,
    AccessChain = 65,
    InBoundsAccessChain = 66,
    PtrAccessChain = 67,
    ArrayLength = 68,
    GenericPtrMemSemantics = 69,
    InBoundsPtrAccessChain = 70,
    Decorate = 71,
    MemberDecorate = 72,
    DecorationGroup = 73,
    GroupDecorate = 74,
    GroupMemberDecorate = 75,
    VectorExtractDynamic = 77,
    VectorInsertDynamic = 78,
    VectorShuffle = 79,
    CompositeConstruct = 80,
    CompositeExtract = 81,
    CompositeInsert = 82,
    CopyObject = 83,
    Transpose = 84,
    SampledImage = 86,
    ImageSampleImplicitLod = 87,
    ImageSampleExplicitLod = 88,
    ImageSampleDrefImplicitLod = 89,
    ImageSampleDrefExplicitLod = 90,
    ImageSampleProjImplicitLod = 91,
    ImageSampleProjExplicitLod = 92,
    ImageSampleProjDrefImplicitLod = 93,
    ImageSampleProjDrefExplicitLod = 94,
    ImageFetch = 95,
    ImageGather = 96,
    ImageDrefGather = 97,
    ImageRead = 98,
    ImageWrite = 99,
    Image = 100,
    ImageQueryFormat = 101,
    ImageQueryOrder = 102,
    ImageQuerySizeLod = 103,
    ImageQuerySize = 104,
    ImageQueryLod = 105,
    ImageQueryLevels = 106,
    ImageQuerySamples = 107,
    ConvertFToU = 109,
    ConvertFToS = 110,
    ConvertSToF = 111,
    ConvertUToF = 112,
    UConvert = 113,
    SConvert = 114,
    FConvert = 115,
    QuantizeToF16 = 116,
    ConvertPtrToU = 117,
    SatConvertSToU = 118,
    SatConvertUToS = 119,
    ConvertUToPtr = 120,
    PtrCastToGeneric = 121,
    GenericCastToPtr = 122,
    GenericCastToPtrExplicit = 123,
    Bitcast = 124,
    SNegate = 126,
    FNegate = 127,
    IAdd = 128,
    FAdd = 129,
    ISub = 130,
    FSub = 131,
    IMul = 132,
    FMul = 133,
    UDiv = 134,
    SDiv = 135,
    FDiv = 136,
    UMod = 137,
    SRem = 138,
    SMod = 139,
    FRem = 140,
    FMod = 141,
    VectorTimesScalar = 142,
    MatrixTimesScalar = 143,
    VectorTimesMatrix = 144,
    MatrixTimesVector = 145,
    MatrixTimesMatrix = 146,
    OuterProduct = 147,
    Dot = 148,
    IAddCarry = 149,
    ISubBorrow = 150,
    UMulExtended = 151,
    SMulExtended = 152,
    Any = 154,
    All = 155,
    IsNan = 156,
    IsInf = 157,
    IsFinite = 158,
    IsNormal = 159,
    SignBitSet = 160,
    LessOrGreater = 161,
    Ordered = 162,
    Unordered = 163,
    LogicalEqual = 164,
    LogicalNotEqual = 165,
    LogicalOr = 166,
    LogicalAnd = 167,
    LogicalNot = 168,
    Select = 169,
    IEqual = 170,
    INotEqual = 171,
    UGreaterThan = 172,
    SGreaterThan = 173,
    UGreaterThanEqual = 174,
    SGreaterThanEqual = 175,
    ULessThan = 176,
    SLessThan = 177,
    ULessThanEqual = 178,
    SLessThanEqual = 179,
    FOrdEqual = 180,
    FUnordEqual = 181,
    FOrdNotEqual = 182,
    FUnordNotEqual = 183,
    FOrdLessThan = 184,
    FUnordLessThan = 185,
    FOrdGreaterThan = 186,
    FUnordGreaterThan = 187,
    FOrdLessThanEqual = 188,
    FUnordLessThanEqual = 189,
    FOrdGreaterThanEqual = 190,
    FUnordGreaterThanEqual = 191,
    ShiftRightLogical = 194,
    ShiftRightArithmetic = 195,
    ShiftLeftLogical = 196,
    BitwiseOr = 197,
    BitwiseXor = 198,
    BitwiseAnd = 199,
    Not = 200,
    BitFieldInsert = 201,
    BitFieldSExtract = 202,
    BitFieldUExtract = 203,
    BitReverse = 204,
    BitCount = 205,
    DPdx = 207,
    DPdy = 208,
    Fwidth = 209,
    DPdxFine = 210,
    DPdyFine = 211,
    FwidthFine = 212,
    DPdxCoarse = 213,
    DPdyCoarse = 214,
    FwidthCoarse = 215,
    EmitVertex = 218,
    EndPrimitive = 219,
    EmitStreamVertex = 220,
    EndStreamPrimitive = 221,
    ControlBarrier = 224,
    MemoryBarrier = 225,
    AtomicLoad = 227,
    AtomicStore = 228,
    AtomicExchange = 229,
    AtomicCompareExchange = 230,
    AtomicCompareExchangeWeak = 231,
    AtomicIIncrement = 232,
    AtomicIDecrement = 233,
    AtomicIAdd = 234,
    AtomicISub = 235,
    AtomicSMin = 236,
    AtomicUMin = 237,
    AtomicSMax = 238,
    AtomicUMax = 239,
    AtomicAnd = 240,
    AtomicOr = 241,
    AtomicXor = 242,
    Phi = 245,
    LoopMerge = 246,
    SelectionMerge = 247,
    Label = 248,
    Branch = 249,
    BranchConditional = 250,
    Switch = 251,
    Kill = 252,
    Return = 253,
    ReturnValue = 254,
    Unreachable = 255,
    LifetimeStart = 256,
    LifetimeStop = 257,
    GroupAsyncCopy = 259,
    GroupWaitEvents = 260,
    GroupAll = 261,
    GroupAny = 262,
    GroupBroadcast = 263,
    GroupIAdd = 264,
    GroupFAdd = 265,
    GroupFMin = 266,
    GroupUMin = 267,
    GroupSMin = 268,
    GroupFMax = 269,
    GroupUMax = 270,
    GroupSMax = 271,
    ReadPipe = 274,
    WritePipe = 275,
    ReservedReadPipe = 276,
    ReservedWritePipe = 277,
    ReserveReadPipePackets = 278,
    ReserveWritePipePackets = 279,
    CommitReadPipe = 280,
    CommitWritePipe = 281,
    IsValidReserveId = 282,
    GetNumPipePackets = 283,
    GetMaxPipePackets = 284,
    GroupReserveReadPipePackets = 285,
    GroupReserveWritePipePackets = 286,
    GroupCommitReadPipe = 287,
    GroupCommitWritePipe = 288,
    EnqueueMarker = 291,
    EnqueueKernel = 292,
    GetKernelNDrangeSubGroupCount = 293,
    GetKernelNDrangeMaxSubGroupSize = 294,
    GetKernelWorkGroupSize = 295,
    GetKernelPreferredWorkGroupSizeMultiple = 296,
    RetainEvent = 297,
    ReleaseEvent = 298,
    CreateUserEvent = 299,
    IsValidEvent = 300,
    SetUserEventStatus = 301,
    CaptureEventProfilingInfo = 302,
    GetDefaultQueue = 303,
    BuildNDRange = 304,
    ImageSparseSampleImplicitLod = 305,
    ImageSparseSampleExplicitLod = 306,
    ImageSparseSampleDrefImplicitLod = 307,
    ImageSparseSampleDrefExplicitLod = 308,
    ImageSparseSampleProjImplicitLod = 309,
    ImageSparseSampleProjExplicitLod = 310,
    ImageSparseSampleProjDrefImplicitLod = 311,
    ImageSparseSampleProjDrefExplicitLod = 312,
    ImageSparseFetch = 313,
    ImageSparseGather = 314,
    ImageSparseDrefGather = 315,
    ImageSparseTexelsResident = 316,
    NoLine = 317,
    AtomicFlagTestAndSet = 318,
    AtomicFlagClear = 319,
    Count
}
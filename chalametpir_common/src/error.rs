use std::{error::Error, fmt::Display};

/// ChalametPIR error codes.
///
/// This enum represents all the possible errors that can occur during the execution of the ChalametPIR protocol.
/// It includes errors related to matrix operations, binary fuse filter operations, and PIR operations.
#[derive(Debug, PartialEq)]
pub enum ChalametPIRError {
    // GPU
    VulkanLibraryNotFound,
    VulkanInstanceCreationFailed,
    VulkanPhysicalDeviceNotFound,
    VulkanDeviceCreationFailed,
    VulkanBufferCreationFailed,
    VulkanCommandBufferBuilderCreationFailed,
    VulkanCommandBufferRecordingFailed,
    VulkanCommandBufferBuildingFailed,
    VulkanCommandBufferExecutionFailed,
    VulkanReadingFromBufferFailed,
    VulkanComputeShaderLoadingFailed,
    VulkanComputePipelineCreationFailed,
    VulkanDescriptorSetCreationFailed,

    // Matrix
    InvalidMatrixDimension,
    IncompatibleDimensionForMatrixMultiplication,
    IncompatibleDimensionForMatrixAddition,
    InvalidNumberOfElementsInMatrix,
    IncompatibleDimensionForRowVectorTransposedMatrixMultiplication,
    InvalidDimensionForVector,
    FailedToDeserializeMatrixFromBytes,

    // Binary Fuse Filter
    EmptyKVDatabase,
    ExhaustedAllAttemptsToBuild3WiseXorFilter(usize),
    ExhaustedAllAttemptsToBuild4WiseXorFilter(usize),
    RowNotDecodable,
    DecodedRowNotPrependedWithDigestOfKey,
    FailedToDeserializeFilterFromBytes,

    // PIR
    KVDatabaseSizeTooLarge,
    InvalidHintMatrix,
    PendingQueryExistsForKey,
    PendingQueryDoesNotExistForKey,
    ArithmeticOverflowAddingQueryIndicator,
    UnsupportedArityForBinaryFuseFilter,
    InvalidResponseVector,
    ImpossibleEncodedDBMatrixElementBitLength,
}

/// Simplify the implementation of Display with macros
macro_rules! display_err {
    ($self:ident, $f:ident, { $($variant:ident => $msg:expr,)* }) => {
        match $self {
            $(Self::$variant => write!($f, $msg),)*
            _ => unreachable!(),
        }
    };
}

impl Display for ChalametPIRError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExhaustedAllAttemptsToBuild3WiseXorFilter(max_num_attempts) => {
                write!(f, "Exhausted {} attempts to build 3-wise XOR binary fuse filter.", max_num_attempts)
            }
            Self::ExhaustedAllAttemptsToBuild4WiseXorFilter(max_num_attempts) => {
                write!(f, "Exhausted {} attempts to build 4-wise XOR binary fuse filter.", max_num_attempts)
            }
            _ => display_err!(self, f, {
                VulkanLibraryNotFound => "Failed to load the default Vulkan library for the system.",
                VulkanInstanceCreationFailed => "Failed to create a new instance of Vulkan.",
                VulkanPhysicalDeviceNotFound => "Failed to find a compatible Vulkan physical device.",
                VulkanDeviceCreationFailed => "Failed to create a Vulkan device and associated queue.",
                VulkanBufferCreationFailed => "Failed to create a Vulkan transfer source buffer.",
                VulkanCommandBufferBuilderCreationFailed => "Failed to create a Vulkan command buffer builder.",
                VulkanCommandBufferRecordingFailed => "Failed to record command in a Vulkan command buffer.",
                VulkanCommandBufferBuildingFailed => "Failed to build a Vulkan command buffer.",
                VulkanCommandBufferExecutionFailed => "Failed to execute the Vulkan command buffer.",
                VulkanReadingFromBufferFailed => "Failed to read from Vulkan buffer.",
                VulkanComputeShaderLoadingFailed => "Failed to load Vulkan compute shader module.",
                VulkanComputePipelineCreationFailed => "Failed to create Vulkan compute pipeline.",
                VulkanDescriptorSetCreationFailed => "Failed to create descriptor set for Vulkan compute pipeline.",
                InvalidMatrixDimension => "The number of rows and columns in the matrix must be non-zero.",
                IncompatibleDimensionForMatrixMultiplication => "The matrix dimensions do not allow multiplication.",
                IncompatibleDimensionForMatrixAddition => "The matrix dimensions do not allow addition.",
                InvalidNumberOfElementsInMatrix => "The matrix must have \"rows * columns\" elements.",
                IncompatibleDimensionForRowVectorTransposedMatrixMultiplication => "The dimensions are incompatible for multiplication of a row vector and a transposed matrix.",
                InvalidDimensionForVector => "A vector must have either one row or one column.",
                FailedToDeserializeMatrixFromBytes => "Matrix deserialization failed.",
                EmptyKVDatabase => "Cannot encode empty key-value database.",
                RowNotDecodable => "Encoded KV database matrix's row cannot be decoded.",
                DecodedRowNotPrependedWithDigestOfKey => "Decoded row does not have the digest of the key prepended to it.",
                FailedToDeserializeFilterFromBytes => "Binary fuse filter deserialization failed.",
                KVDatabaseSizeTooLarge => "The key-value database is too large; it can have a maximum of 2^42 entries.",
                InvalidHintMatrix => "Unexpected number of rows in the hint matrix.",
                PendingQueryExistsForKey => "A pending query for this key was found in the internal client state.",
                PendingQueryDoesNotExistForKey => "No pending query for this key exists in the internal client state.",
                ArithmeticOverflowAddingQueryIndicator => "Encountered arithmetic overflow while adding the query indicator to the query vector 'b'.",
                UnsupportedArityForBinaryFuseFilter => "Binary Fuse Filter supports arity of either 3 or 4.",
                InvalidResponseVector => "Unexpected dimension of the response vector.",
            }),
        }
    }
}

impl Error for ChalametPIRError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

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
/// simplify with macros
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
            },
            Self::ExhaustedAllAttemptsToBuild4WiseXorFilter(max_num_attempts) => {
                write!(f, "Exhausted {} attempts to build 4-wise XOR binary fuse filter.", max_num_attempts)
            },
            _ => display_simple!(self, f, {
                Self::VulkanLibraryNotFound => "Failed to load the default Vulkan library for the system.",
                Self::VulkanInstanceCreationFailed => "Failed to create a new instance of Vulkan.",
                Self::VulkanPhysicalDeviceNotFound => "Failed to find a compatible Vulkan physical device.",
                Self::VulkanDeviceCreationFailed => "Failed to create a Vulkan device and associated queue.",
                Self::VulkanBufferCreationFailed => "Failed to create a Vulkan transfer source buffer.",
                Self::VulkanCommandBufferBuilderCreationFailed => "Failed to create a Vulkan command buffer builder.",
                Self::VulkanCommandBufferRecordingFailed => "Failed to record command in a Vulkan command buffer.",
                Self::VulkanCommandBufferBuildingFailed => "Failed to build a Vulkan command buffer.",
                Self::VulkanCommandBufferExecutionFailed => "Failed to execute the Vulkan command buffer.",
                Self::VulkanReadingFromBufferFailed => "Failed to read from Vulkan buuffer.",
                Self::VulkanComputeShaderLoadingFailed => "Failed to load Vulkan compute shader module.",
                Self::VulkanComputePipelineCreationFailed => "Failed to create Vulkan compute pipeline.",
                Self::VulkanDescriptorSetCreationFailed => "Failed to create descriptor set for Vulkan compute pipeline.",

                Self::InvalidMatrixDimension => "The number of rows and columns in the matrix must be non-zero.",
                Self::IncompatibleDimensionForMatrixMultiplication => "The matrix dimensions do not allow multiplication.",
                Self::IncompatibleDimensionForMatrixAddition => "The matrix dimensions do not allow addition.",
                Self::InvalidNumberOfElementsInMatrix => "The matrix must have \"rows * columns\" elements.",
                Self::IncompatibleDimensionForRowVectorTransposedMatrixMultiplication => 
                    "The dimensions are incompatible for multiplication of a row vector and a transposed matrix."
                Self::InvalidDimensionForVector => "A vector must have either one row or one column.",
                Self::FailedToDeserializeMatrixFromBytes => "Matrix deserialization failed",

                Self::EmptyKVDatabase => "Cannot encode empty key-value database.",
                Self::RowNotDecodable => "Encoded KV database matrix's row cannot be decoded.",
                Self::DecodedRowNotPrependedWithDigestOfKey => "Decoded row does not have the digest of the key prepended to it.",
                Self::FailedToDeserializeFilterFromBytes => "Binary fuse filter deserialization failed",

                Self::KVDatabaseSizeTooLarge => "The key-value database is too large; it can have a maximum of 2^42 entries.",
                Self::InvalidHintMatrix => "Unexpected number of rows in the hint matrix.",
                Self::PendingQueryExistsForKey => "A pending query for this key was found in the internal client state.",
                Self::PendingQueryDoesNotExistForKey => "No pending query for this key exists in the internal client state.",
                Self::ArithmeticOverflowAddingQueryIndicator => 
                    "Encountered arithmetic overflow while adding the query indicator to the query vector 'b'."
                Self::UnsupportedArityForBinaryFuseFilter => "Binary Fuse Filter supports arity of either 3 or 4.",
                Self::InvalidResponseVector => "Unexpected dimension of the response vector.",
                Self::ImpossibleEncodedDBMatrixElementBitLength => "Encoded database matrix's element bit length mustn't ever exceed 16.",
            }),
        }
    }
}

impl Error for ChalametPIRError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

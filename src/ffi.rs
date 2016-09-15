use libc::*;

pub enum pthreadpool {}
pub type pthreadpool_t = *mut pthreadpool;

#[derive(Clone, Copy, Debug)]
#[must_use]
#[repr(C)]
pub enum nnp_status {
  /** The call succeeded, and all output arguments now contain valid data. */
  nnp_status_success = 0,
  /** NNPACK function was called with batch_size == 0. */
  nnp_status_invalid_batch_size = 2,
  /** NNPACK function was called with channels == 0. */
  nnp_status_invalid_channels = 3,
  /** NNPACK function was called with input_channels == 0. */
  nnp_status_invalid_input_channels = 4,
  /** NNPACK function was called with output_channels == 0. */
  nnp_status_invalid_output_channels = 5,
  /** NNPACK function was called with input_size.height == 0 or input_size.width == 0 */
  nnp_status_invalid_input_size = 10,
  /** NNPACK function was called with input_stride.height == 0 or input_stride.width == 0 */
  nnp_status_invalid_input_stride = 11,
  /** NNPACK function was called with input_padding not less than respective kernel (or pooling) size, i.e.:
   *
   *  - input_padding.left   >= kernel_size.width  (>= pooling_size.width)
   *  - input_padding.right  >= kernel_size.width  (>= pooling_size.width)
   *  - input_padding.top    >= kernel_size.height (>= pooling_size.height)
   *  - input_padding.bottom >= kernel_size.height (>= pooling_size.height)
   */
  nnp_status_invalid_input_padding = 12,
  /** NNPACK function was called with kernel_size.height == 0 or kernel_size.width == 0 */
  nnp_status_invalid_kernel_size = 13,
  /** NNPACK function was called with pooling_size.height == 0 or pooling_size.width == 0 */
  nnp_status_invalid_pooling_size = 14,
  /** NNPACK function was called with pooling_stride.height == 0 or pooling_stride.width == 0 */
  nnp_status_invalid_pooling_stride = 15,
  /** NNPACK function was called with convolution algorithm not in nnp_convolution_algorithm enumeration */
  nnp_status_invalid_algorithm = 16,
  /** NNPACK function was called with convolution transform strategy not in nnp_convolution_transform_strategy enum */
  nnp_status_invalid_transform_strategy = 17,
  /** NNPACK function was called with output_subsampling.height == 0 or output_subsampling.width == 0 */
  //nnp_status_invalid_output_subsampling = 13,

  /** NNPACK does not support the particular input size for the function */
  nnp_status_unsupported_input_size = 20,
  /** NNPACK does not support the particular input stride for the function */
  nnp_status_unsupported_input_stride = 21,
  /** NNPACK does not support the particular input padding for the function */
  nnp_status_unsupported_input_padding = 22,
  /** NNPACK does not support the particular kernel size for the function */
  nnp_status_unsupported_kernel_size = 23,
  /** NNPACK does not support the particular pooling size for the function */
  nnp_status_unsupported_pooling_size = 24,
  /** NNPACK does not support the particular pooling stride for the function */
  nnp_status_unsupported_pooling_stride = 25,
  /** NNPACK does not support the particular convolution algorithm for the function */
  nnp_status_unsupported_algorithm = 26,
  /** NNPACK does not support the particular convolution transform strategy for the algorithm */
  nnp_status_unsupported_transform_strategy = 27,

  /** NNPACK function was called before the library was initialized */
  nnp_status_uninitialized = 50,
  /** NNPACK does not implement this function for the host CPU */
  nnp_status_unsupported_hardware = 51,
  /** NNPACK failed to allocate memory for temporary buffers */
  nnp_status_out_of_memory = 52
}

impl nnp_status {
  pub fn is_ok(&self) -> bool {
    match *self {
      nnp_status::nnp_status_success => true,
      _ => false,
    }
  }

  pub fn is_err(&self) -> bool {
    !self.is_ok()
  }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum nnp_convolution_algorithm {
  /** Let NNPACK choose the algorithm depending on layer parameters */
  nnp_convolution_algorithm_auto = 0,
  /** Tiled convolution based on 2D Fourier transform with 8x8 blocks. Supports kernels up to 8x8. */
  nnp_convolution_algorithm_ft8x8 = 1,
  /** Tiled convolution based on 2D Fourier transform with 16x16 blocks. Supports kernels up to 16x16. */
  nnp_convolution_algorithm_ft16x16 = 2,
  /** Tiled convolution based on 2D Winograd transform F(3x3, 6x6) with 8x8 blocks. Supports only 3x3 kernels. */
  nnp_convolution_algorithm_wt8x8 = 3,
  /** Direct convolution via implicit GEMM. */
  nnp_convolution_algorithm_implicit_gemm = 4,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum nnp_convolution_transform_strategy {
  nnp_convolution_transform_strategy_block_based = 1,
  nnp_convolution_transform_strategy_tuple_based = 2,
  nnp_convolution_transform_strategy_precomputed = 3,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct nnp_size {
  /** Width (horizontal size) of an image, kernel, or pooling filter. */
  pub width: size_t,
  /** Height (vertical size) of an image, kernel, or pooling filter. */
  pub height: size_t,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct nnp_padding {
  /** Padding above the image data */
  pub top: size_t,
  /** Padding on the right of image data */
  pub right: size_t,
  /** Padding below the image data */
  pub bottom: size_t,
  /** Padding on the left of image data */
  pub left: size_t,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct nnp_profile {
  /** Time spent inside the function call, in seconds. */
  pub total: f64,
  /** Time spend on transformation of the input or input gradient tensor, in seconds. */
  pub input_transform: f64,
  /** Time spend on transformation of the kernel or kernel gradient tensor, in seconds. */
  pub kernel_transform: f64,
  /** Time spend on transformation of the output or output gradient tensor, in seconds. */
  pub output_transform: f64,
  /** Time spend on multiplication-accumulation of transformed coefficients, in seconds. */
  pub block_multiplication: f64,
}

#[link(name = "nnpack_native", kind = "static")]
extern "C" {
  pub fn pthreadpool_create(threads_count: size_t) -> pthreadpool_t;
  pub fn pthreadpool_get_threads_count(threadpool: pthreadpool_t) -> size_t;
  pub fn pthreadpool_destroy(threadpool: pthreadpool_t);

  pub fn nnp_initialize() -> nnp_status;
  pub fn nnp_deinitialize() -> nnp_status;
  pub fn nnp_convolution_output(
      algorithm: nnp_convolution_algorithm,
      batch_size: size_t,
      input_channels: size_t,
      output_channels: size_t,
      input_size: nnp_size,
      input_padding: nnp_padding,
      kernel_size: nnp_size,
      input: *const f32,
      kernel: *const f32,
      bias: *const f32,
      output: *mut f32,
      threadpool: pthreadpool_t,
      profile: *mut nnp_profile,
  ) -> nnp_status;
  pub fn nnp_convolution_input_gradient(
      algorithm: nnp_convolution_algorithm,
      batch_size: size_t,
      input_channels: size_t,
      output_channels: size_t,
      input_size: nnp_size,
      input_padding: nnp_padding,
      kernel_size: nnp_size,
      grad_output: *const f32,
      kernel: *const f32,
      grad_input: *mut f32,
      threadpool: pthreadpool_t,
      profile: *mut nnp_profile,
  ) -> nnp_status;
  pub fn nnp_convolution_kernel_gradient(
      algorithm: nnp_convolution_algorithm,
      batch_size: size_t,
      input_channels: size_t,
      output_channels: size_t,
      input_size: nnp_size,
      input_padding: nnp_padding,
      kernel_size: nnp_size,
      input: *const f32,
      grad_output: *const f32,
      grad_kernel: *mut f32,
      threadpool: pthreadpool_t,
      profile: *mut nnp_profile,
  ) -> nnp_status;
  pub fn nnp_convolution_inference(
      algorithm: nnp_convolution_algorithm,
      transform_strategy: nnp_convolution_transform_strategy,
      input_channels: size_t,
      output_channels: size_t,
      input_size: nnp_size,
      input_padding: nnp_padding,
      kernel_size: nnp_size,
      output_subsampling: nnp_size,
      input: *const f32,
      kernel: *const f32,
      bias: *const f32,
      output: *mut f32,
      threadpool: pthreadpool_t,
      profile: *mut nnp_profile,
  ) -> nnp_status;
  pub fn nnp_fully_connected_output(
      batch_size: size_t,
      input_channels: size_t,
      output_channels: size_t,
      input: *const f32,
      kernel: *const f32,
      output: *mut f32,
      threadpool: pthreadpool_t,
      profile: *mut nnp_profile,
  ) -> nnp_status;
  pub fn nnp_fully_connected_inference(
      input_channels: size_t,
      output_channels: size_t,
      input: *const f32,
      kernel: *const f32,
      output: *mut f32,
      threadpool: pthreadpool_t,
  ) -> nnp_status;
  pub fn nnp_max_pooling_output() -> nnp_status;
  pub fn nnp_softmax_output() -> nnp_status;
  pub fn nnp_relu_output() -> nnp_status;
  pub fn nnp_relu_input_gradient() -> nnp_status;
}

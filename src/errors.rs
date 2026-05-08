use ash::vk;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum OutOfMemoryError {
  #[error("Out of device memory")]
  OutOfDeviceMemory,
  #[error("Out of host memory")]
  OutOfHostMemory,
}

impl From<vk::Result> for OutOfMemoryError {
  fn from(value: vk::Result) -> Self {
    match value {
      vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => OutOfMemoryError::OutOfDeviceMemory,
      vk::Result::ERROR_OUT_OF_HOST_MEMORY => OutOfMemoryError::OutOfHostMemory,
      _ => {
        panic!("Invalid vk::Result to OutOfMemoryError cast: {:?}", value);
      }
    }
  }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("Vulkan returned ERROR_DEVICE_LOST. See https://docs.vulkan.org/spec/latest/chapters/devsandqueues.html#devsandqueues-lost-device")]
pub struct DeviceIsLost;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum QueueSubmitError {
  #[error(transparent)]
  OutOfMemory(#[from] OutOfMemoryError),
  #[error(transparent)]
  DeviceIsLost(#[from] DeviceIsLost),
}

impl From<vk::Result> for QueueSubmitError {
  fn from(value: vk::Result) -> Self {
    match value {
      vk::Result::ERROR_OUT_OF_DEVICE_MEMORY | vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
        QueueSubmitError::OutOfMemory(value.into())
      }
      vk::Result::ERROR_DEVICE_LOST => QueueSubmitError::DeviceIsLost(DeviceIsLost {}),
      _ => {
        panic!("Invalid vk::Result to QueueSubmitError cast: {:?}", value);
      }
    }
  }
}

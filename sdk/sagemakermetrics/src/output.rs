// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchPutMetricsOutput {
    /// <p>Lists any errors that occur when inserting metric data.</p>
    #[doc(hidden)]
    pub errors: std::option::Option<std::vec::Vec<crate::model::BatchPutMetricsError>>,
}
impl BatchPutMetricsOutput {
    /// <p>Lists any errors that occur when inserting metric data.</p>
    pub fn errors(&self) -> std::option::Option<&[crate::model::BatchPutMetricsError]> {
        self.errors.as_deref()
    }
}
/// See [`BatchPutMetricsOutput`](crate::output::BatchPutMetricsOutput).
pub mod batch_put_metrics_output {

    /// A builder for [`BatchPutMetricsOutput`](crate::output::BatchPutMetricsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) errors: std::option::Option<std::vec::Vec<crate::model::BatchPutMetricsError>>,
    }
    impl Builder {
        /// Appends an item to `errors`.
        ///
        /// To override the contents of this collection use [`set_errors`](Self::set_errors).
        ///
        /// <p>Lists any errors that occur when inserting metric data.</p>
        pub fn errors(mut self, input: crate::model::BatchPutMetricsError) -> Self {
            let mut v = self.errors.unwrap_or_default();
            v.push(input);
            self.errors = Some(v);
            self
        }
        /// <p>Lists any errors that occur when inserting metric data.</p>
        pub fn set_errors(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchPutMetricsError>>,
        ) -> Self {
            self.errors = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchPutMetricsOutput`](crate::output::BatchPutMetricsOutput).
        pub fn build(self) -> crate::output::BatchPutMetricsOutput {
            crate::output::BatchPutMetricsOutput {
                errors: self.errors,
            }
        }
    }
}
impl BatchPutMetricsOutput {
    /// Creates a new builder-style object to manufacture [`BatchPutMetricsOutput`](crate::output::BatchPutMetricsOutput).
    pub fn builder() -> crate::output::batch_put_metrics_output::Builder {
        crate::output::batch_put_metrics_output::Builder::default()
    }
}

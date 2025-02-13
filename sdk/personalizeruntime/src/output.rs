// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetRecommendationsOutput {
    /// <p>A list of recommendations sorted in descending order by prediction score. There can be a maximum of 500 items in the list.</p>
    #[doc(hidden)]
    pub item_list: std::option::Option<std::vec::Vec<crate::model::PredictedItem>>,
    /// <p>The ID of the recommendation.</p>
    #[doc(hidden)]
    pub recommendation_id: std::option::Option<std::string::String>,
}
impl GetRecommendationsOutput {
    /// <p>A list of recommendations sorted in descending order by prediction score. There can be a maximum of 500 items in the list.</p>
    pub fn item_list(&self) -> std::option::Option<&[crate::model::PredictedItem]> {
        self.item_list.as_deref()
    }
    /// <p>The ID of the recommendation.</p>
    pub fn recommendation_id(&self) -> std::option::Option<&str> {
        self.recommendation_id.as_deref()
    }
}
/// See [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput).
pub mod get_recommendations_output {

    /// A builder for [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) item_list: std::option::Option<std::vec::Vec<crate::model::PredictedItem>>,
        pub(crate) recommendation_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `item_list`.
        ///
        /// To override the contents of this collection use [`set_item_list`](Self::set_item_list).
        ///
        /// <p>A list of recommendations sorted in descending order by prediction score. There can be a maximum of 500 items in the list.</p>
        pub fn item_list(mut self, input: crate::model::PredictedItem) -> Self {
            let mut v = self.item_list.unwrap_or_default();
            v.push(input);
            self.item_list = Some(v);
            self
        }
        /// <p>A list of recommendations sorted in descending order by prediction score. There can be a maximum of 500 items in the list.</p>
        pub fn set_item_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::PredictedItem>>,
        ) -> Self {
            self.item_list = input;
            self
        }
        /// <p>The ID of the recommendation.</p>
        pub fn recommendation_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.recommendation_id = Some(input.into());
            self
        }
        /// <p>The ID of the recommendation.</p>
        pub fn set_recommendation_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.recommendation_id = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput).
        pub fn build(self) -> crate::output::GetRecommendationsOutput {
            crate::output::GetRecommendationsOutput {
                item_list: self.item_list,
                recommendation_id: self.recommendation_id,
            }
        }
    }
}
impl GetRecommendationsOutput {
    /// Creates a new builder-style object to manufacture [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput).
    pub fn builder() -> crate::output::get_recommendations_output::Builder {
        crate::output::get_recommendations_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetPersonalizedRankingOutput {
    /// <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
    #[doc(hidden)]
    pub personalized_ranking: std::option::Option<std::vec::Vec<crate::model::PredictedItem>>,
    /// <p>The ID of the recommendation.</p>
    #[doc(hidden)]
    pub recommendation_id: std::option::Option<std::string::String>,
}
impl GetPersonalizedRankingOutput {
    /// <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
    pub fn personalized_ranking(&self) -> std::option::Option<&[crate::model::PredictedItem]> {
        self.personalized_ranking.as_deref()
    }
    /// <p>The ID of the recommendation.</p>
    pub fn recommendation_id(&self) -> std::option::Option<&str> {
        self.recommendation_id.as_deref()
    }
}
/// See [`GetPersonalizedRankingOutput`](crate::output::GetPersonalizedRankingOutput).
pub mod get_personalized_ranking_output {

    /// A builder for [`GetPersonalizedRankingOutput`](crate::output::GetPersonalizedRankingOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) personalized_ranking:
            std::option::Option<std::vec::Vec<crate::model::PredictedItem>>,
        pub(crate) recommendation_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `personalized_ranking`.
        ///
        /// To override the contents of this collection use [`set_personalized_ranking`](Self::set_personalized_ranking).
        ///
        /// <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
        pub fn personalized_ranking(mut self, input: crate::model::PredictedItem) -> Self {
            let mut v = self.personalized_ranking.unwrap_or_default();
            v.push(input);
            self.personalized_ranking = Some(v);
            self
        }
        /// <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
        pub fn set_personalized_ranking(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::PredictedItem>>,
        ) -> Self {
            self.personalized_ranking = input;
            self
        }
        /// <p>The ID of the recommendation.</p>
        pub fn recommendation_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.recommendation_id = Some(input.into());
            self
        }
        /// <p>The ID of the recommendation.</p>
        pub fn set_recommendation_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.recommendation_id = input;
            self
        }
        /// Consumes the builder and constructs a [`GetPersonalizedRankingOutput`](crate::output::GetPersonalizedRankingOutput).
        pub fn build(self) -> crate::output::GetPersonalizedRankingOutput {
            crate::output::GetPersonalizedRankingOutput {
                personalized_ranking: self.personalized_ranking,
                recommendation_id: self.recommendation_id,
            }
        }
    }
}
impl GetPersonalizedRankingOutput {
    /// Creates a new builder-style object to manufacture [`GetPersonalizedRankingOutput`](crate::output::GetPersonalizedRankingOutput).
    pub fn builder() -> crate::output::get_personalized_ranking_output::Builder {
        crate::output::get_personalized_ranking_output::Builder::default()
    }
}

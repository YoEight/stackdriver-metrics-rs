/// A single strongly-typed value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// The typed value field.
    #[prost(oneof = "typed_value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<typed_value::Value>,
}
/// Nested message and enum types in `TypedValue`.
pub mod typed_value {
    /// The typed value field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A Boolean value: `true` or `false`.
        #[prost(bool, tag = "1")]
        BoolValue(bool),
        /// A 64-bit integer. Its range is approximately &plusmn;9.2x10<sup>18</sup>.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A 64-bit double-precision floating-point number. Its magnitude
        /// is approximately &plusmn;10<sup>&plusmn;300</sup> and it has 16
        /// significant digits of precision.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// A variable-length string value.
        #[prost(string, tag = "4")]
        StringValue(::prost::alloc::string::String),
        /// A distribution value.
        #[prost(message, tag = "5")]
        DistributionValue(super::super::super::api::Distribution),
    }
}
/// A closed time interval. It extends from the start time to the end time, and includes both: `[startTime, endTime]`. Valid time intervals depend on the [`MetricKind`](/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors#MetricKind) of the metric value. The end time must not be earlier than the start time. When writing data points, the start time must not be more than 25 hours in the past and the end time must not be more than five minutes in the future.
///
/// * For `GAUGE` metrics, the `startTime` value is technically optional; if
///   no value is specified, the start time defaults to the value of the
///   end time, and the interval represents a single point in time. If both
///   start and end times are specified, they must be identical. Such an
///   interval is valid only for `GAUGE` metrics, which are point-in-time
///   measurements. The end time of a new interval must be at least a
///   millisecond after the end time of the previous interval.
///
/// * For `DELTA` metrics, the start time and end time must specify a
///   non-zero interval, with subsequent points specifying contiguous and
///   non-overlapping intervals. For `DELTA` metrics, the start time of
///   the next interval must be at least a millisecond after the end time
///   of the previous interval.
///
/// * For `CUMULATIVE` metrics, the start time and end time must specify a
///   a non-zero interval, with subsequent points specifying the same
///   start time and increasing end times, until an event resets the
///   cumulative value to zero and sets a new start time for the following
///   points. The new start time must be at least a millisecond after the
///   end time of the previous interval.
///
/// * The start time of a new interval must be at least a millisecond after the
///   end time of the previous interval because intervals are closed. If the
///   start time of a new interval is the same as the end time of the previous
///   interval, then data written at the new start time could overwrite data
///   written at the previous end time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// Required. The end of the time interval.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The beginning of the time interval.  The default value
    /// for the start time is the end time. The start time must not be
    /// later than the end time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes how to combine multiple time series to provide a different view of
/// the data.  Aggregation of time series is done in two steps. First, each time
/// series in the set is _aligned_ to the same time interval boundaries, then the
/// set of time series is optionally _reduced_ in number.
///
/// Alignment consists of applying the `per_series_aligner` operation
/// to each time series after its data has been divided into regular
/// `alignment_period` time intervals. This process takes _all_ of the data
/// points in an alignment period, applies a mathematical transformation such as
/// averaging, minimum, maximum, delta, etc., and converts them into a single
/// data point per period.
///
/// Reduction is when the aligned and transformed time series can optionally be
/// combined, reducing the number of time series through similar mathematical
/// transformations. Reduction involves applying a `cross_series_reducer` to
/// all the time series, optionally sorting the time series into subsets with
/// `group_by_fields`, and applying the reducer to each subset.
///
/// The raw time series data can contain a huge amount of information from
/// multiple sources. Alignment and reduction transforms this mass of data into
/// a more manageable and representative collection of data, for example "the
/// 95% latency across the average of all tasks in a cluster". This
/// representative data can be more easily graphed and comprehended, and the
/// individual time series data is still available for later drilldown. For more
/// details, see [Filtering and
/// aggregation](https://cloud.google.com/monitoring/api/v3/aggregation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregation {
    /// The `alignment_period` specifies a time interval, in seconds, that is used
    /// to divide the data in all the
    /// [time series][google.monitoring.v3.TimeSeries] into consistent blocks of
    /// time. This will be done before the per-series aligner can be applied to
    /// the data.
    ///
    /// The value must be at least 60 seconds. If a per-series
    /// aligner other than `ALIGN_NONE` is specified, this field is required or an
    /// error is returned. If no per-series aligner is specified, or the aligner
    /// `ALIGN_NONE` is specified, then this field is ignored.
    ///
    /// The maximum value of the `alignment_period` is 104 weeks (2 years) for
    /// charts, and 90,000 seconds (25 hours) for alerting policies.
    #[prost(message, optional, tag = "1")]
    pub alignment_period: ::core::option::Option<::prost_types::Duration>,
    /// An `Aligner` describes how to bring the data points in a single
    /// time series into temporal alignment. Except for `ALIGN_NONE`, all
    /// alignments cause all the data points in an `alignment_period` to be
    /// mathematically grouped together, resulting in a single data point for
    /// each `alignment_period` with end timestamp at the end of the period.
    ///
    /// Not all alignment operations may be applied to all time series. The valid
    /// choices depend on the `metric_kind` and `value_type` of the original time
    /// series. Alignment can change the `metric_kind` or the `value_type` of
    /// the time series.
    ///
    /// Time series data must be aligned in order to perform cross-time
    /// series reduction. If `cross_series_reducer` is specified, then
    /// `per_series_aligner` must be specified and not equal to `ALIGN_NONE`
    /// and `alignment_period` must be specified; otherwise, an error is
    /// returned.
    #[prost(enumeration = "aggregation::Aligner", tag = "2")]
    pub per_series_aligner: i32,
    /// The reduction operation to be used to combine time series into a single
    /// time series, where the value of each data point in the resulting series is
    /// a function of all the already aligned values in the input time series.
    ///
    /// Not all reducer operations can be applied to all time series. The valid
    /// choices depend on the `metric_kind` and the `value_type` of the original
    /// time series. Reduction can yield a time series with a different
    /// `metric_kind` or `value_type` than the input time series.
    ///
    /// Time series data must first be aligned (see `per_series_aligner`) in order
    /// to perform cross-time series reduction. If `cross_series_reducer` is
    /// specified, then `per_series_aligner` must be specified, and must not be
    /// `ALIGN_NONE`. An `alignment_period` must also be specified; otherwise, an
    /// error is returned.
    #[prost(enumeration = "aggregation::Reducer", tag = "4")]
    pub cross_series_reducer: i32,
    /// The set of fields to preserve when `cross_series_reducer` is
    /// specified. The `group_by_fields` determine how the time series are
    /// partitioned into subsets prior to applying the aggregation
    /// operation. Each subset contains time series that have the same
    /// value for each of the grouping fields. Each individual time
    /// series is a member of exactly one subset. The
    /// `cross_series_reducer` is applied to each subset of time series.
    /// It is not possible to reduce across different resource types, so
    /// this field implicitly contains `resource.type`.  Fields not
    /// specified in `group_by_fields` are aggregated away.  If
    /// `group_by_fields` is not specified and all the time series have
    /// the same resource type, then the time series are aggregated into
    /// a single output time series. If `cross_series_reducer` is not
    /// defined, this field is ignored.
    #[prost(string, repeated, tag = "5")]
    pub group_by_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Aggregation`.
pub mod aggregation {
    /// The `Aligner` specifies the operation that will be applied to the data
    /// points in each alignment period in a time series. Except for
    /// `ALIGN_NONE`, which specifies that no operation be applied, each alignment
    /// operation replaces the set of data values in each alignment period with
    /// a single value: the result of applying the operation to the data values.
    /// An aligned time series has a single data value at the end of each
    /// `alignment_period`.
    ///
    /// An alignment operation can change the data type of the values, too. For
    /// example, if you apply a counting operation to boolean values, the data
    /// `value_type` in the original time series is `BOOLEAN`, but the `value_type`
    /// in the aligned result is `INT64`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Aligner {
        /// No alignment. Raw data is returned. Not valid if cross-series reduction
        /// is requested. The `value_type` of the result is the same as the
        /// `value_type` of the input.
        AlignNone = 0,
        /// Align and convert to
        /// [DELTA][google.api.MetricDescriptor.MetricKind.DELTA].
        /// The output is `delta = y1 - y0`.
        ///
        /// This alignment is valid for
        /// [CUMULATIVE][google.api.MetricDescriptor.MetricKind.CUMULATIVE] and
        /// `DELTA` metrics. If the selected alignment period results in periods
        /// with no data, then the aligned value for such a period is created by
        /// interpolation. The `value_type`  of the aligned result is the same as
        /// the `value_type` of the input.
        AlignDelta = 1,
        /// Align and convert to a rate. The result is computed as
        /// `rate = (y1 - y0)/(t1 - t0)`, or "delta over time".
        /// Think of this aligner as providing the slope of the line that passes
        /// through the value at the start and at the end of the `alignment_period`.
        ///
        /// This aligner is valid for `CUMULATIVE`
        /// and `DELTA` metrics with numeric values. If the selected alignment
        /// period results in periods with no data, then the aligned value for
        /// such a period is created by interpolation. The output is a `GAUGE`
        /// metric with `value_type` `DOUBLE`.
        ///
        /// If, by "rate", you mean "percentage change", see the
        /// `ALIGN_PERCENT_CHANGE` aligner instead.
        AlignRate = 2,
        /// Align by interpolating between adjacent points around the alignment
        /// period boundary. This aligner is valid for `GAUGE` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as the
        /// `value_type` of the input.
        AlignInterpolate = 3,
        /// Align by moving the most recent data point before the end of the
        /// alignment period to the boundary at the end of the alignment
        /// period. This aligner is valid for `GAUGE` metrics. The `value_type` of
        /// the aligned result is the same as the `value_type` of the input.
        AlignNextOlder = 4,
        /// Align the time series by returning the minimum value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as
        /// the `value_type` of the input.
        AlignMin = 10,
        /// Align the time series by returning the maximum value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as
        /// the `value_type` of the input.
        AlignMax = 11,
        /// Align the time series by returning the mean value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is `DOUBLE`.
        AlignMean = 12,
        /// Align the time series by returning the number of values in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric or Boolean values. The `value_type` of the aligned result is
        /// `INT64`.
        AlignCount = 13,
        /// Align the time series by returning the sum of the values in each
        /// alignment period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with numeric and distribution values. The `value_type` of the
        /// aligned result is the same as the `value_type` of the input.
        AlignSum = 14,
        /// Align the time series by returning the standard deviation of the values
        /// in each alignment period. This aligner is valid for `GAUGE` and
        /// `DELTA` metrics with numeric values. The `value_type` of the output is
        /// `DOUBLE`.
        AlignStddev = 15,
        /// Align the time series by returning the number of `True` values in
        /// each alignment period. This aligner is valid for `GAUGE` metrics with
        /// Boolean values. The `value_type` of the output is `INT64`.
        AlignCountTrue = 16,
        /// Align the time series by returning the number of `False` values in
        /// each alignment period. This aligner is valid for `GAUGE` metrics with
        /// Boolean values. The `value_type` of the output is `INT64`.
        AlignCountFalse = 24,
        /// Align the time series by returning the ratio of the number of `True`
        /// values to the total number of values in each alignment period. This
        /// aligner is valid for `GAUGE` metrics with Boolean values. The output
        /// value is in the range [0.0, 1.0] and has `value_type` `DOUBLE`.
        AlignFractionTrue = 17,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 99th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile99 = 18,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 95th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile95 = 19,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 50th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile50 = 20,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 5th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile05 = 21,
        /// Align and convert to a percentage change. This aligner is valid for
        /// `GAUGE` and `DELTA` metrics with numeric values. This alignment returns
        /// `((current - previous)/previous) * 100`, where the value of `previous` is
        /// determined based on the `alignment_period`.
        ///
        /// If the values of `current` and `previous` are both 0, then the returned
        /// value is 0. If only `previous` is 0, the returned value is infinity.
        ///
        /// A 10-minute moving mean is computed at each point of the alignment period
        /// prior to the above calculation to smooth the metric and prevent false
        /// positives from very short-lived spikes. The moving mean is only
        /// applicable for data whose values are `>= 0`. Any values `< 0` are
        /// treated as a missing datapoint, and are ignored. While `DELTA`
        /// metrics are accepted by this alignment, special care should be taken that
        /// the values for the metric will always be positive. The output is a
        /// `GAUGE` metric with `value_type` `DOUBLE`.
        AlignPercentChange = 23,
    }
    /// A Reducer operation describes how to aggregate data points from multiple
    /// time series into a single time series, where the value of each data point
    /// in the resulting series is a function of all the already aligned values in
    /// the input time series.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reducer {
        /// No cross-time series reduction. The output of the `Aligner` is
        /// returned.
        ReduceNone = 0,
        /// Reduce by computing the mean value across time series for each
        /// alignment period. This reducer is valid for
        /// [DELTA][google.api.MetricDescriptor.MetricKind.DELTA] and
        /// [GAUGE][google.api.MetricDescriptor.MetricKind.GAUGE] metrics with
        /// numeric or distribution values. The `value_type` of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ReduceMean = 1,
        /// Reduce by computing the minimum value across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric values. The `value_type` of the output is the same as the
        /// `value_type` of the input.
        ReduceMin = 2,
        /// Reduce by computing the maximum value across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric values. The `value_type` of the output is the same as the
        /// `value_type` of the input.
        ReduceMax = 3,
        /// Reduce by computing the sum across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric and distribution values. The `value_type` of the output is
        /// the same as the `value_type` of the input.
        ReduceSum = 4,
        /// Reduce by computing the standard deviation across time series
        /// for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics with numeric or distribution values. The `value_type`
        /// of the output is `DOUBLE`.
        ReduceStddev = 5,
        /// Reduce by computing the number of data points across time series
        /// for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of numeric, Boolean, distribution, and string
        /// `value_type`. The `value_type` of the output is `INT64`.
        ReduceCount = 6,
        /// Reduce by computing the number of `True`-valued data points across time
        /// series for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of Boolean `value_type`. The `value_type` of the output
        /// is `INT64`.
        ReduceCountTrue = 7,
        /// Reduce by computing the number of `False`-valued data points across time
        /// series for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of Boolean `value_type`. The `value_type` of the output
        /// is `INT64`.
        ReduceCountFalse = 15,
        /// Reduce by computing the ratio of the number of `True`-valued data points
        /// to the total number of data points for each alignment period. This
        /// reducer is valid for `DELTA` and `GAUGE` metrics of Boolean `value_type`.
        /// The output value is in the range [0.0, 1.0] and has `value_type`
        /// `DOUBLE`.
        ReduceFractionTrue = 8,
        /// Reduce by computing the [99th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile99 = 9,
        /// Reduce by computing the [95th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile95 = 10,
        /// Reduce by computing the [50th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile50 = 11,
        /// Reduce by computing the [5th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile05 = 12,
    }
}
/// Specifies an ordering relationship on two arguments, called `left` and
/// `right`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComparisonType {
    /// No ordering relationship is specified.
    ComparisonUnspecified = 0,
    /// True if the left argument is greater than the right argument.
    ComparisonGt = 1,
    /// True if the left argument is greater than or equal to the right argument.
    ComparisonGe = 2,
    /// True if the left argument is less than the right argument.
    ComparisonLt = 3,
    /// True if the left argument is less than or equal to the right argument.
    ComparisonLe = 4,
    /// True if the left argument is equal to the right argument.
    ComparisonEq = 5,
    /// True if the left argument is not equal to the right argument.
    ComparisonNe = 6,
}
/// The tier of service for a Workspace. Please see the
/// [service tiers
/// documentation](https://cloud.google.com/monitoring/workspaces/tiers) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceTier {
    /// An invalid sentinel value, used to indicate that a tier has not
    /// been provided explicitly.
    Unspecified = 0,
    /// The Stackdriver Basic tier, a free tier of service that provides basic
    /// features, a moderate allotment of logs, and access to built-in metrics.
    /// A number of features are not available in this tier. For more details,
    /// see [the service tiers
    /// documentation](https://cloud.google.com/monitoring/workspaces/tiers).
    Basic = 1,
    /// The Stackdriver Premium tier, a higher, more expensive tier of service
    /// that provides access to all Stackdriver features, lets you use Stackdriver
    /// with AWS accounts, and has a larger allotments for logs and metrics. For
    /// more details, see [the service tiers
    /// documentation](https://cloud.google.com/monitoring/workspaces/tiers).
    Premium = 2,
}
/// Describes a change made to a configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutationRecord {
    /// When the change occurred.
    #[prost(message, optional, tag = "1")]
    pub mutate_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The email address of the user making the change.
    #[prost(string, tag = "2")]
    pub mutated_by: ::prost::alloc::string::String,
}
/// A description of the conditions under which some aspect of your system is
/// considered to be "unhealthy" and the ways to notify people or services about
/// this state. For an overview of alert policies, see
/// [Introduction to Alerting](https://cloud.google.com/monitoring/alerts/).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertPolicy {
    /// Required if the policy exists. The resource name for this policy. The
    /// format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID]
    ///
    /// `[ALERT_POLICY_ID]` is assigned by Stackdriver Monitoring when the policy
    /// is created.  When calling the
    /// [alertPolicies.create][google.monitoring.v3.AlertPolicyService.CreateAlertPolicy]
    /// method, do not include the `name` field in the alerting policy passed as
    /// part of the request.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A short name or phrase used to identify the policy in dashboards,
    /// notifications, and incidents. To avoid confusion, don't use the same
    /// display name for multiple policies in the same project. The name is
    /// limited to 512 Unicode characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Documentation that is included with notifications and incidents related to
    /// this policy. Best practice is for the documentation to include information
    /// to help responders understand, mitigate, escalate, and correct the
    /// underlying problems detected by the alerting policy. Notification channels
    /// that have limited capacity might not show this documentation.
    #[prost(message, optional, tag = "13")]
    pub documentation: ::core::option::Option<alert_policy::Documentation>,
    /// User-supplied key/value data to be used for organizing and
    /// identifying the `AlertPolicy` objects.
    ///
    /// The field can contain up to 64 entries. Each key and value is limited to
    /// 63 Unicode characters or 128 bytes, whichever is smaller. Labels and
    /// values can contain only lowercase letters, numerals, underscores, and
    /// dashes. Keys must begin with a letter.
    #[prost(map = "string, string", tag = "16")]
    pub user_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A list of conditions for the policy. The conditions are combined by AND or
    /// OR according to the `combiner` field. If the combined conditions evaluate
    /// to true, then an incident is created. A policy can have from one to six
    /// conditions.
    /// If `condition_time_series_query_language` is present, it must be the only
    /// `condition`.
    #[prost(message, repeated, tag = "12")]
    pub conditions: ::prost::alloc::vec::Vec<alert_policy::Condition>,
    /// How to combine the results of multiple conditions to determine if an
    /// incident should be opened.
    /// If `condition_time_series_query_language` is present, this must be
    /// `COMBINE_UNSPECIFIED`.
    #[prost(enumeration = "alert_policy::ConditionCombinerType", tag = "6")]
    pub combiner: i32,
    /// Whether or not the policy is enabled. On write, the default interpretation
    /// if unset is that the policy is enabled. On read, clients should not make
    /// any assumption about the state if it has not been populated. The
    /// field should always be populated on List and Get operations, unless
    /// a field projection has been specified that strips it out.
    #[prost(message, optional, tag = "17")]
    pub enabled: ::core::option::Option<bool>,
    /// Read-only description of how the alert policy is invalid. OK if the alert
    /// policy is valid. If not OK, the alert policy will not generate incidents.
    #[prost(message, optional, tag = "18")]
    pub validity: ::core::option::Option<super::super::rpc::Status>,
    /// Identifies the notification channels to which notifications should be sent
    /// when incidents are opened or closed or when new violations occur on
    /// an already opened incident. Each element of this array corresponds to
    /// the `name` field in each of the
    /// [`NotificationChannel`][google.monitoring.v3.NotificationChannel]
    /// objects that are returned from the [`ListNotificationChannels`]
    /// [google.monitoring.v3.NotificationChannelService.ListNotificationChannels]
    /// method. The format of the entries in this field is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]
    #[prost(string, repeated, tag = "14")]
    pub notification_channels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A read-only record of the creation of the alerting policy. If provided
    /// in a call to create or update, this field will be ignored.
    #[prost(message, optional, tag = "10")]
    pub creation_record: ::core::option::Option<MutationRecord>,
    /// A read-only record of the most recent change to the alerting policy. If
    /// provided in a call to create or update, this field will be ignored.
    #[prost(message, optional, tag = "11")]
    pub mutation_record: ::core::option::Option<MutationRecord>,
}
/// Nested message and enum types in `AlertPolicy`.
pub mod alert_policy {
    /// A content string and a MIME type that describes the content string's
    /// format.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Documentation {
        /// The text of the documentation, interpreted according to `mime_type`.
        /// The content may not exceed 8,192 Unicode characters and may not exceed
        /// more than 10,240 bytes when encoded in UTF-8 format, whichever is
        /// smaller.
        #[prost(string, tag = "1")]
        pub content: ::prost::alloc::string::String,
        /// The format of the `content` field. Presently, only the value
        /// `"text/markdown"` is supported. See
        /// [Markdown](https://en.wikipedia.org/wiki/Markdown) for more information.
        #[prost(string, tag = "2")]
        pub mime_type: ::prost::alloc::string::String,
    }
    /// A condition is a true/false test that determines when an alerting policy
    /// should open an incident. If a condition evaluates to true, it signifies
    /// that something is wrong.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Condition {
        /// Required if the condition exists. The unique resource name for this
        /// condition. Its format is:
        ///
        ///     projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]
        ///
        /// `[CONDITION_ID]` is assigned by Stackdriver Monitoring when the
        /// condition is created as part of a new or updated alerting policy.
        ///
        /// When calling the
        /// [alertPolicies.create][google.monitoring.v3.AlertPolicyService.CreateAlertPolicy]
        /// method, do not include the `name` field in the conditions of the
        /// requested alerting policy. Stackdriver Monitoring creates the
        /// condition identifiers and includes them in the new policy.
        ///
        /// When calling the
        /// [alertPolicies.update][google.monitoring.v3.AlertPolicyService.UpdateAlertPolicy]
        /// method to update a policy, including a condition `name` causes the
        /// existing condition to be updated. Conditions without names are added to
        /// the updated policy. Existing conditions are deleted if they are not
        /// updated.
        ///
        /// Best practice is to preserve `[CONDITION_ID]` if you make only small
        /// changes, such as those to condition thresholds, durations, or trigger
        /// values.  Otherwise, treat the change as a new condition and let the
        /// existing condition be deleted.
        #[prost(string, tag = "12")]
        pub name: ::prost::alloc::string::String,
        /// A short name or phrase used to identify the condition in dashboards,
        /// notifications, and incidents. To avoid confusion, don't use the same
        /// display name for multiple conditions in the same policy.
        #[prost(string, tag = "6")]
        pub display_name: ::prost::alloc::string::String,
        /// Only one of the following condition types will be specified.
        #[prost(oneof = "condition::Condition", tags = "1, 2, 19")]
        pub condition: ::core::option::Option<condition::Condition>,
    }
    /// Nested message and enum types in `Condition`.
    pub mod condition {
        /// Specifies how many time series must fail a predicate to trigger a
        /// condition. If not specified, then a `{count: 1}` trigger is used.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Trigger {
            /// A type of trigger.
            #[prost(oneof = "trigger::Type", tags = "1, 2")]
            pub r#type: ::core::option::Option<trigger::Type>,
        }
        /// Nested message and enum types in `Trigger`.
        pub mod trigger {
            /// A type of trigger.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// The absolute number of time series that must fail
                /// the predicate for the condition to be triggered.
                #[prost(int32, tag = "1")]
                Count(i32),
                /// The percentage of time series that must fail the
                /// predicate for the condition to be triggered.
                #[prost(double, tag = "2")]
                Percent(f64),
            }
        }
        /// A condition type that compares a collection of time series
        /// against a threshold.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetricThreshold {
            /// Required. A [filter](https://cloud.google.com/monitoring/api/v3/filters) that
            /// identifies which time series should be compared with the threshold.
            ///
            /// The filter is similar to the one that is specified in the
            /// [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)
            /// (that call is useful to verify the time series that will be retrieved /
            /// processed). The filter must specify the metric type and the resource
            /// type. Optionally, it can specify resource labels and metric labels.
            /// This field must not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "2")]
            pub filter: ::prost::alloc::string::String,
            /// Specifies the alignment of data points in individual time series as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resrouces). Multiple aggregations
            /// are applied in the order specified.
            ///
            /// This field is similar to the one in the [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
            /// It is advisable to use the `ListTimeSeries` method when debugging this
            /// field.
            #[prost(message, repeated, tag = "8")]
            pub aggregations: ::prost::alloc::vec::Vec<super::super::Aggregation>,
            /// A [filter](https://cloud.google.com/monitoring/api/v3/filters) that
            /// identifies a time series that should be used as the denominator of a
            /// ratio that will be compared with the threshold. If a
            /// `denominator_filter` is specified, the time series specified by the
            /// `filter` field will be used as the numerator.
            ///
            /// The filter must specify the metric type and optionally may contain
            /// restrictions on resource type, resource labels, and metric labels.
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "9")]
            pub denominator_filter: ::prost::alloc::string::String,
            /// Specifies the alignment of data points in individual time series
            /// selected by `denominatorFilter` as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resources).
            ///
            /// When computing ratios, the `aggregations` and
            /// `denominator_aggregations` fields must use the same alignment period
            /// and produce time series that have the same periodicity and labels.
            #[prost(message, repeated, tag = "10")]
            pub denominator_aggregations: ::prost::alloc::vec::Vec<super::super::Aggregation>,
            /// The comparison to apply between the time series (indicated by `filter`
            /// and `aggregation`) and the threshold (indicated by `threshold_value`).
            /// The comparison is applied on each time series, with the time series
            /// on the left-hand side and the threshold on the right-hand side.
            ///
            /// Only `COMPARISON_LT` and `COMPARISON_GT` are supported currently.
            #[prost(enumeration = "super::super::ComparisonType", tag = "4")]
            pub comparison: i32,
            /// A value against which to compare the time series.
            #[prost(double, tag = "5")]
            pub threshold_value: f64,
            /// The amount of time that a time series must violate the
            /// threshold to be considered failing. Currently, only values
            /// that are a multiple of a minute--e.g., 0, 60, 120, or 300
            /// seconds--are supported. If an invalid value is given, an
            /// error will be returned. When choosing a duration, it is useful to
            /// keep in mind the frequency of the underlying time series data
            /// (which may also be affected by any alignments specified in the
            /// `aggregations` field); a good duration is long enough so that a single
            /// outlier does not generate spurious alerts, but short enough that
            /// unhealthy states are detected and alerted on quickly.
            #[prost(message, optional, tag = "6")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`,
            /// or by the ratio, if `denominator_filter` and `denominator_aggregations`
            /// are specified.
            #[prost(message, optional, tag = "7")]
            pub trigger: ::core::option::Option<Trigger>,
        }
        /// A condition type that checks that monitored resources
        /// are reporting data. The configuration defines a metric and
        /// a set of monitored resources. The predicate is considered in violation
        /// when a time series for the specified metric of a monitored
        /// resource does not include any data in the specified `duration`.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetricAbsence {
            /// Required. A [filter](https://cloud.google.com/monitoring/api/v3/filters) that
            /// identifies which time series should be compared with the threshold.
            ///
            /// The filter is similar to the one that is specified in the
            /// [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)
            /// (that call is useful to verify the time series that will be retrieved /
            /// processed). The filter must specify the metric type and the resource
            /// type. Optionally, it can specify resource labels and metric labels.
            /// This field must not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "1")]
            pub filter: ::prost::alloc::string::String,
            /// Specifies the alignment of data points in individual time series as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resrouces). Multiple aggregations
            /// are applied in the order specified.
            ///
            /// This field is similar to the one in the [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
            /// It is advisable to use the `ListTimeSeries` method when debugging this
            /// field.
            #[prost(message, repeated, tag = "5")]
            pub aggregations: ::prost::alloc::vec::Vec<super::super::Aggregation>,
            /// The amount of time that a time series must fail to report new
            /// data to be considered failing. The minimum value of this field
            /// is 120 seconds. Larger values that are a multiple of a
            /// minute--for example, 240 or 300 seconds--are supported.
            /// If an invalid value is given, an
            /// error will be returned. The `Duration.nanos` field is
            /// ignored.
            #[prost(message, optional, tag = "2")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`.
            #[prost(message, optional, tag = "3")]
            pub trigger: ::core::option::Option<Trigger>,
        }
        /// A condition type that allows alert policies to be defined using
        /// [Monitoring Query Language](https://cloud.google.com/monitoring/mql).
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MonitoringQueryLanguageCondition {
            /// [Monitoring Query Language](https://cloud.google.com/monitoring/mql)
            /// query that outputs a boolean stream.
            #[prost(string, tag = "1")]
            pub query: ::prost::alloc::string::String,
            /// The amount of time that a time series must violate the
            /// threshold to be considered failing. Currently, only values
            /// that are a multiple of a minute--e.g., 0, 60, 120, or 300
            /// seconds--are supported. If an invalid value is given, an
            /// error will be returned. When choosing a duration, it is useful to
            /// keep in mind the frequency of the underlying time series data
            /// (which may also be affected by any alignments specified in the
            /// `aggregations` field); a good duration is long enough so that a single
            /// outlier does not generate spurious alerts, but short enough that
            /// unhealthy states are detected and alerted on quickly.
            #[prost(message, optional, tag = "2")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`,
            /// or by the ratio, if `denominator_filter` and `denominator_aggregations`
            /// are specified.
            #[prost(message, optional, tag = "3")]
            pub trigger: ::core::option::Option<Trigger>,
        }
        /// Only one of the following condition types will be specified.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Condition {
            /// A condition that compares a time series against a threshold.
            #[prost(message, tag = "1")]
            ConditionThreshold(MetricThreshold),
            /// A condition that checks that a time series continues to
            /// receive new data points.
            #[prost(message, tag = "2")]
            ConditionAbsent(MetricAbsence),
            /// A condition that uses the Monitoring Query Language to define
            /// alerts.
            #[prost(message, tag = "19")]
            ConditionMonitoringQueryLanguage(MonitoringQueryLanguageCondition),
        }
    }
    /// Operators for combining conditions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConditionCombinerType {
        /// An unspecified combiner.
        CombineUnspecified = 0,
        /// Combine conditions using the logical `AND` operator. An
        /// incident is created only if all the conditions are met
        /// simultaneously. This combiner is satisfied if all conditions are
        /// met, even if they are met on completely different resources.
        And = 1,
        /// Combine conditions using the logical `OR` operator. An incident
        /// is created if any of the listed conditions is met.
        Or = 2,
        /// Combine conditions using logical `AND` operator, but unlike the regular
        /// `AND` option, an incident is created only if all conditions are met
        /// simultaneously on at least one resource.
        AndWithMatchingResource = 3,
    }
}
/// A single data point in a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// The time interval to which the data point applies.  For `GAUGE` metrics,
    /// the start time is optional, but if it is supplied, it must equal the
    /// end time.  For `DELTA` metrics, the start
    /// and end time should specify a non-zero interval, with subsequent points
    /// specifying contiguous and non-overlapping intervals.  For `CUMULATIVE`
    /// metrics, the start and end time should specify a non-zero interval, with
    /// subsequent points specifying the same start time and increasing end times,
    /// until an event resets the cumulative value to zero and sets a new start
    /// time for the following points.
    #[prost(message, optional, tag = "1")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// The value of the data point.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<TypedValue>,
}
/// A collection of data points that describes the time-varying values
/// of a metric. A time series is identified by a combination of a
/// fully-specified monitored resource and a fully-specified metric.
/// This type is used for both listing and creating time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeries {
    /// The associated metric. A fully-specified metric used to identify the time
    /// series.
    #[prost(message, optional, tag = "1")]
    pub metric: ::core::option::Option<super::super::api::Metric>,
    /// The associated monitored resource.  Custom metrics can use only certain
    /// monitored resource types in their time series data.
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<super::super::api::MonitoredResource>,
    /// Output only. The associated monitored resource metadata. When reading a
    /// time series, this field will include metadata labels that are explicitly
    /// named in the reduction. When creating a time series, this field is ignored.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::core::option::Option<super::super::api::MonitoredResourceMetadata>,
    /// The metric kind of the time series. When listing time series, this metric
    /// kind might be different from the metric kind of the associated metric if
    /// this time series is an alignment or reduction of other time series.
    ///
    /// When creating a time series, this field is optional. If present, it must be
    /// the same as the metric kind of the associated metric. If the associated
    /// metric's descriptor must be auto-created, then this field specifies the
    /// metric kind of the new descriptor and must be either `GAUGE` (the default)
    /// or `CUMULATIVE`.
    #[prost(
        enumeration = "super::super::api::metric_descriptor::MetricKind",
        tag = "3"
    )]
    pub metric_kind: i32,
    /// The value type of the time series. When listing time series, this value
    /// type might be different from the value type of the associated metric if
    /// this time series is an alignment or reduction of other time series.
    ///
    /// When creating a time series, this field is optional. If present, it must be
    /// the same as the type of the data in the `points` field.
    #[prost(
        enumeration = "super::super::api::metric_descriptor::ValueType",
        tag = "4"
    )]
    pub value_type: i32,
    /// The data points of this time series. When listing time series, points are
    /// returned in reverse time order.
    ///
    /// When creating a time series, this field must contain exactly one point and
    /// the point's type must be the same as the value type of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// the value type of the descriptor is determined by the point's type, which
    /// must be `BOOL`, `INT64`, `DOUBLE`, or `DISTRIBUTION`.
    #[prost(message, repeated, tag = "5")]
    pub points: ::prost::alloc::vec::Vec<Point>,
    /// The units in which the metric value is reported. It is only applicable
    /// if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit`
    /// defines the representation of the stored metric values.
    #[prost(string, tag = "8")]
    pub unit: ::prost::alloc::string::String,
}
/// A descriptor for the labels and points in a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesDescriptor {
    /// Descriptors for the labels.
    #[prost(message, repeated, tag = "1")]
    pub label_descriptors: ::prost::alloc::vec::Vec<super::super::api::LabelDescriptor>,
    /// Descriptors for the point data value columns.
    #[prost(message, repeated, tag = "5")]
    pub point_descriptors: ::prost::alloc::vec::Vec<time_series_descriptor::ValueDescriptor>,
}
/// Nested message and enum types in `TimeSeriesDescriptor`.
pub mod time_series_descriptor {
    /// A descriptor for the value columns in a data point.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueDescriptor {
        /// The value key.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The value type.
        #[prost(
            enumeration = "super::super::super::api::metric_descriptor::ValueType",
            tag = "2"
        )]
        pub value_type: i32,
        /// The value stream kind.
        #[prost(
            enumeration = "super::super::super::api::metric_descriptor::MetricKind",
            tag = "3"
        )]
        pub metric_kind: i32,
        /// The unit in which `time_series` point values are reported. `unit`
        /// follows the UCUM format for units as seen in
        /// https://unitsofmeasure.org/ucum.html.
        /// `unit` is only valid if `value_type` is INTEGER, DOUBLE, DISTRIBUTION.
        #[prost(string, tag = "4")]
        pub unit: ::prost::alloc::string::String,
    }
}
/// Represents the values of a time series associated with a
/// TimeSeriesDescriptor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesData {
    /// The values of the labels in the time series identifier, given in the same
    /// order as the `label_descriptors` field of the TimeSeriesDescriptor
    /// associated with this object. Each value must have a value of the type
    /// given in the corresponding entry of `label_descriptors`.
    #[prost(message, repeated, tag = "1")]
    pub label_values: ::prost::alloc::vec::Vec<LabelValue>,
    /// The points in the time series.
    #[prost(message, repeated, tag = "2")]
    pub point_data: ::prost::alloc::vec::Vec<time_series_data::PointData>,
}
/// Nested message and enum types in `TimeSeriesData`.
pub mod time_series_data {
    /// A point's value columns and time interval. Each point has one or more
    /// point values corresponding to the entries in `point_descriptors` field in
    /// the TimeSeriesDescriptor associated with this object.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PointData {
        /// The values that make up the point.
        #[prost(message, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<super::TypedValue>,
        /// The time interval associated with the point.
        #[prost(message, optional, tag = "2")]
        pub time_interval: ::core::option::Option<super::TimeInterval>,
    }
}
/// A label value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelValue {
    /// The label value can be a bool, int64, or string.
    #[prost(oneof = "label_value::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<label_value::Value>,
}
/// Nested message and enum types in `LabelValue`.
pub mod label_value {
    /// The label value can be a bool, int64, or string.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A bool label value.
        #[prost(bool, tag = "1")]
        BoolValue(bool),
        /// An int64 label value.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A string label value.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
    }
}
/// An error associated with a query in the time series query language format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryError {
    /// The location of the time series query language text that this error applies
    /// to.
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<TextLocator>,
    /// The error message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// A locator for text. Indicates a particular part of the text of a request or
/// of an object referenced in the request.
///
/// For example, suppose the request field `text` contains:
///
///   text: "The quick brown fox jumps over the lazy dog."
///
/// Then the locator:
///
///   source: "text"
///   start_position {
///     line: 1
///     column: 17
///   }
///   end_position {
///     line: 1
///     column: 19
///   }
///
/// refers to the part of the text: "fox".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLocator {
    /// The source of the text. The source may be a field in the request, in which
    /// case its format is the format of the
    /// google.rpc.BadRequest.FieldViolation.field field in
    /// https://cloud.google.com/apis/design/errors#error_details. It may also be
    /// be a source other than the request field (e.g. a macro definition
    /// referenced in the text of the query), in which case this is the name of
    /// the source (e.g. the macro name).
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// The position of the first byte within the text.
    #[prost(message, optional, tag = "2")]
    pub start_position: ::core::option::Option<text_locator::Position>,
    /// The position of the last byte within the text.
    #[prost(message, optional, tag = "3")]
    pub end_position: ::core::option::Option<text_locator::Position>,
    /// If `source`, `start_position`, and `end_position` describe a call on
    /// some object (e.g. a macro in the time series query language text) and a
    /// location is to be designated in that object's text, `nested_locator`
    /// identifies the location within that object.
    #[prost(message, optional, boxed, tag = "4")]
    pub nested_locator: ::core::option::Option<::prost::alloc::boxed::Box<TextLocator>>,
    /// When `nested_locator` is set, this field gives the reason for the nesting.
    /// Usually, the reason is a macro invocation. In that case, the macro name
    /// (including the leading '@') signals the location of the macro call
    /// in the text and a macro argument name (including the leading '$') signals
    /// the location of the macro argument inside the macro body that got
    /// substituted away.
    #[prost(string, tag = "5")]
    pub nesting_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TextLocator`.
pub mod text_locator {
    /// The position of a byte within the text.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        /// The line, starting with 1, where the byte is positioned.
        #[prost(int32, tag = "1")]
        pub line: i32,
        /// The column within the line, starting with 1, where the byte is
        /// positioned. This is a byte index even though the text is UTF-8.
        #[prost(int32, tag = "2")]
        pub column: i32,
    }
}
/// The `ListMonitoredResourceDescriptors` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// An optional [filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// describing the descriptors to be returned.  The filter can reference the
    /// descriptor's type and labels. For example, the following filter returns
    /// only Google Compute Engine descriptors that have an `id` label:
    ///
    ///     resource.type = starts_with("gce_") AND resource.label:id
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListMonitoredResourceDescriptors` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// The monitored resource descriptors that are available to this project
    /// and that match `filter`, if present.
    #[prost(message, repeated, tag = "1")]
    pub resource_descriptors:
        ::prost::alloc::vec::Vec<super::super::api::MonitoredResourceDescriptor>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `GetMonitoredResourceDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonitoredResourceDescriptorRequest {
    /// Required. The monitored resource descriptor to get.  The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/monitoredResourceDescriptors/[RESOURCE_TYPE]
    ///
    /// The `[RESOURCE_TYPE]` is a predefined type, such as
    /// `cloudsql_database`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `ListMetricDescriptors` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricDescriptorsRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// If this field is empty, all custom and
    /// system-defined metric descriptors are returned.
    /// Otherwise, the [filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifies which metric descriptors are to be
    /// returned. For example, the following filter matches all
    /// [custom metrics](https://cloud.google.com/monitoring/custom-metrics):
    ///
    ///     metric.type = starts_with("custom.googleapis.com/")
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListMetricDescriptors` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricDescriptorsResponse {
    /// The metric descriptors that are available to the project
    /// and that match the value of `filter`, if present.
    #[prost(message, repeated, tag = "1")]
    pub metric_descriptors: ::prost::alloc::vec::Vec<super::super::api::MetricDescriptor>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `GetMetricDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricDescriptorRequest {
    /// Required. The metric descriptor on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/metricDescriptors/[METRIC_ID]
    ///
    /// An example value of `[METRIC_ID]` is
    /// `"compute.googleapis.com/instance/disk/read_bytes_count"`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `CreateMetricDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetricDescriptorRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new [custom metric](https://cloud.google.com/monitoring/custom-metrics)
    /// descriptor.
    #[prost(message, optional, tag = "2")]
    pub metric_descriptor: ::core::option::Option<super::super::api::MetricDescriptor>,
}
/// The `DeleteMetricDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMetricDescriptorRequest {
    /// Required. The metric descriptor on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/metricDescriptors/[METRIC_ID]
    ///
    /// An example of `[METRIC_ID]` is:
    /// `"custom.googleapis.com/my_test_metric"`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `ListTimeSeries` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTimeSeriesRequest {
    /// Required. The project, organization or folder on which to execute the request. The
    /// format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///     organizations/[ORGANIZATION_ID]
    ///     folders/[FOLDER_ID]
    #[prost(string, tag = "10")]
    pub name: ::prost::alloc::string::String,
    /// Required. A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// that specifies which time series should be returned.  The filter must
    /// specify a single metric type, and can additionally specify metric labels
    /// and other information. For example:
    ///
    ///     metric.type = "compute.googleapis.com/instance/cpu/usage_time" AND
    ///         metric.labels.instance_name = "my-instance-name"
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. The time interval for which results should be returned. Only time series
    /// that contain data points in the specified interval are included
    /// in the response.
    #[prost(message, optional, tag = "4")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// Specifies the alignment of data points in individual time series as
    /// well as how to combine the retrieved time series across specified labels.
    ///
    /// By default (if no `aggregation` is explicitly specified), the raw time
    /// series data is returned.
    #[prost(message, optional, tag = "5")]
    pub aggregation: ::core::option::Option<Aggregation>,
    /// Apply a second aggregation after `aggregation` is applied. May only be
    /// specified if `aggregation` is specified.
    #[prost(message, optional, tag = "11")]
    pub secondary_aggregation: ::core::option::Option<Aggregation>,
    /// Unsupported: must be left blank. The points in each time series are
    /// currently returned in reverse time order (most recent to oldest).
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
    /// Required. Specifies which information is returned about the time series.
    #[prost(enumeration = "list_time_series_request::TimeSeriesView", tag = "7")]
    pub view: i32,
    /// A positive number that is the maximum number of results to return. If
    /// `page_size` is empty or more than 100,000 results, the effective
    /// `page_size` is 100,000 results. If `view` is set to `FULL`, this is the
    /// maximum number of `Points` returned. If `view` is set to `HEADERS`, this is
    /// the maximum number of `TimeSeries` returned.
    #[prost(int32, tag = "8")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "9")]
    pub page_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListTimeSeriesRequest`.
pub mod list_time_series_request {
    /// Controls which fields are returned by `ListTimeSeries`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeSeriesView {
        /// Returns the identity of the metric(s), the time series,
        /// and the time series data.
        Full = 0,
        /// Returns the identity of the metric and the time series resource,
        /// but not the time series data.
        Headers = 1,
    }
}
/// The `ListTimeSeries` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTimeSeriesResponse {
    /// One or more time series that match the filter included in the request.
    #[prost(message, repeated, tag = "1")]
    pub time_series: ::prost::alloc::vec::Vec<TimeSeries>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Query execution errors that may have caused the time series data returned
    /// to be incomplete.
    #[prost(message, repeated, tag = "3")]
    pub execution_errors: ::prost::alloc::vec::Vec<super::super::rpc::Status>,
    /// The unit in which all `time_series` point values are reported. `unit`
    /// follows the UCUM format for units as seen in
    /// https://unitsofmeasure.org/ucum.html.
    /// If different `time_series` have different units (for example, because they
    /// come from different metric types, or a unit is absent), then `unit` will be
    /// "{not_a_unit}".
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
}
/// The `CreateTimeSeries` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new data to be added to a list of time series.
    /// Adds at most one data point to each of several time series.  The new data
    /// point must be more recent than any other point in its time series.  Each
    /// `TimeSeries` value must fully specify a unique time series by supplying
    /// all label values for the metric and the monitored resource.
    ///
    /// The maximum number of `TimeSeries` objects per `Create` request is 200.
    #[prost(message, repeated, tag = "2")]
    pub time_series: ::prost::alloc::vec::Vec<TimeSeries>,
}
/// DEPRECATED. Used to hold per-time-series error status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesError {
    /// DEPRECATED. Time series ID that resulted in the `status` error.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub time_series: ::core::option::Option<TimeSeries>,
    /// DEPRECATED. The status of the requested write operation for `time_series`.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
}
/// Summary of the result of a failed request to write data to a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesSummary {
    /// The number of points in the request.
    #[prost(int32, tag = "1")]
    pub total_point_count: i32,
    /// The number of points that were successfully written.
    #[prost(int32, tag = "2")]
    pub success_point_count: i32,
    /// The number of points that failed to be written. Order is not guaranteed.
    #[prost(message, repeated, tag = "3")]
    pub errors: ::prost::alloc::vec::Vec<create_time_series_summary::Error>,
}
/// Nested message and enum types in `CreateTimeSeriesSummary`.
pub mod create_time_series_summary {
    /// Detailed information about an error category.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// The status of the requested write operation.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<super::super::super::rpc::Status>,
        /// The number of points that couldn't be written because of `status`.
        #[prost(int32, tag = "2")]
        pub point_count: i32,
    }
}
/// The `QueryTimeSeries` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeSeriesRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The query in the [Monitoring Query
    /// Language](https://cloud.google.com/monitoring/mql/reference) format.
    /// The default time zone is in UTC.
    #[prost(string, tag = "7")]
    pub query: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of time_series_data to return.
    #[prost(int32, tag = "9")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "10")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `QueryTimeSeries` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeSeriesResponse {
    /// The descriptor for the time series data.
    #[prost(message, optional, tag = "8")]
    pub time_series_descriptor: ::core::option::Option<TimeSeriesDescriptor>,
    /// The time series data.
    #[prost(message, repeated, tag = "9")]
    pub time_series_data: ::prost::alloc::vec::Vec<TimeSeriesData>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results, use that value as
    /// `page_token` in the next call to this method.
    #[prost(string, tag = "10")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Query execution errors that may have caused the time series data returned
    /// to be incomplete. The available data will be available in the
    /// response.
    #[prost(message, repeated, tag = "11")]
    pub partial_errors: ::prost::alloc::vec::Vec<super::super::rpc::Status>,
}
/// This is an error detail intended to be used with INVALID_ARGUMENT errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorList {
    /// Errors in parsing the time series query language text. The number of errors
    /// in the response may be limited.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<QueryError>,
    /// A summary of all the errors.
    #[prost(string, tag = "2")]
    pub error_summary: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod metric_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages metric descriptors, monitored resource descriptors, and"]
    #[doc = " time series data."]
    #[derive(Debug, Clone)]
    pub struct MetricServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MetricServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MetricServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MetricServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MetricServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists monitored resource descriptors that match a filter. This method does not require a Workspace."]
        pub async fn list_monitored_resource_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMonitoredResourceDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListMonitoredResourceDescriptorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/ListMonitoredResourceDescriptors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single monitored resource descriptor. This method does not require a Workspace."]
        pub async fn get_monitored_resource_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMonitoredResourceDescriptorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::api::MonitoredResourceDescriptor>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/GetMonitoredResourceDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists metric descriptors that match a filter. This method does not require a Workspace."]
        pub async fn list_metric_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetricDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListMetricDescriptorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/ListMetricDescriptors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single metric descriptor. This method does not require a Workspace."]
        pub async fn get_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricDescriptorRequest>,
        ) -> Result<tonic::Response<super::super::super::api::MetricDescriptor>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/GetMetricDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new metric descriptor."]
        #[doc = " User-created metric descriptors define"]
        #[doc = " [custom metrics](https://cloud.google.com/monitoring/custom-metrics)."]
        pub async fn create_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetricDescriptorRequest>,
        ) -> Result<tonic::Response<super::super::super::api::MetricDescriptor>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/CreateMetricDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a metric descriptor. Only user-created"]
        #[doc = " [custom metrics](https://cloud.google.com/monitoring/custom-metrics) can be"]
        #[doc = " deleted."]
        pub async fn delete_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMetricDescriptorRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/DeleteMetricDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists time series that match a filter. This method does not require a Workspace."]
        pub async fn list_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTimeSeriesRequest>,
        ) -> Result<tonic::Response<super::ListTimeSeriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/ListTimeSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or adds data to one or more time series."]
        #[doc = " The response is empty if all time series in the request were written."]
        #[doc = " If any time series could not be written, a corresponding failure message is"]
        #[doc = " included in the error response."]
        pub async fn create_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTimeSeriesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/CreateTimeSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}

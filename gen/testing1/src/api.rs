use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;
use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Testing related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// use testing1::api::TestMatrix;
/// use testing1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestMatrix::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_create(req, "projectId")
///              .request_id("At")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Testing<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<S>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Testing<S> {}

impl<'a, S> Testing<S> {

    pub fn new(client: hyper::Client<S, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<S>) -> Testing<S> {
        Testing {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/4.0.1".to_string(),
            _base_url: "https://testing.googleapis.com/".to_string(),
            _root_url: "https://testing.googleapis.com/".to_string(),
        }
    }

    pub fn application_detail_service(&'a self) -> ApplicationDetailServiceMethods<'a, S> {
        ApplicationDetailServiceMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, S> {
        ProjectMethods { hub: &self }
    }
    pub fn test_environment_catalog(&'a self) -> TestEnvironmentCatalogMethods<'a, S> {
        TestEnvironmentCatalogMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/4.0.1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://testing.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://testing.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Identifies an account and how to log into it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// An automatic google login account.
    #[serde(rename="googleAuto")]
    pub google_auto: Option<GoogleAuto>,
}

impl client::Part for Account {}


/// A single Android device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidDevice {
    /// Required. The id of the Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidModelId")]
    pub android_model_id: Option<String>,
    /// Required. The id of the Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidVersionId")]
    pub android_version_id: Option<String>,
    /// Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options.
    pub locale: Option<String>,
    /// Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientation: Option<String>,
}

impl client::Part for AndroidDevice {}


/// The currently supported Android devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidDeviceCatalog {
    /// The set of supported Android device models.
    pub models: Option<Vec<AndroidModel>>,
    /// The set of supported runtime configurations.
    #[serde(rename="runtimeConfiguration")]
    pub runtime_configuration: Option<AndroidRuntimeConfiguration>,
    /// The set of supported Android OS versions.
    pub versions: Option<Vec<AndroidVersion>>,
}

impl client::Part for AndroidDeviceCatalog {}


/// A list of Android device configurations in which the test is to be executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidDeviceList {
    /// Required. A list of Android devices.
    #[serde(rename="androidDevices")]
    pub android_devices: Option<Vec<AndroidDevice>>,
}

impl client::Part for AndroidDeviceList {}


/// A test of an Android application that can control an Android component independently of its normal lifecycle. Android instrumentation tests run an application APK and test APK inside the same process on a virtual or physical AndroidDevice. They also specify a test runner class, such as com.google.GoogleTestRunner, which can vary on the specific instrumentation framework chosen. See for more information on types of Android tests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidInstrumentationTest {
    /// The APK for the application under test.
    #[serde(rename="appApk")]
    pub app_apk: Option<FileReference>,
    /// A multi-apk app bundle for the application under test.
    #[serde(rename="appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The java package for the application under test. The default value is determined by examining the application's manifest.
    #[serde(rename="appPackageId")]
    pub app_package_id: Option<String>,
    /// The option of whether running each test within its own invocation of instrumentation with Android Test Orchestrator or not. ** Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or higher! ** Orchestrator offers the following benefits: - No shared state - Crashes are isolated - Logs are scoped per test See for more information about Android Test Orchestrator. If not set, the test will be run without the orchestrator.
    #[serde(rename="orchestratorOption")]
    pub orchestrator_option: Option<String>,
    /// The option to run tests in multiple shards in parallel.
    #[serde(rename="shardingOption")]
    pub sharding_option: Option<ShardingOption>,
    /// Required. The APK containing the test code to be executed.
    #[serde(rename="testApk")]
    pub test_apk: Option<FileReference>,
    /// The java package for the test to be executed. The default value is determined by examining the application's manifest.
    #[serde(rename="testPackageId")]
    pub test_package_id: Option<String>,
    /// The InstrumentationTestRunner class. The default value is determined by examining the application's manifest.
    #[serde(rename="testRunnerClass")]
    pub test_runner_class: Option<String>,
    /// Each target must be fully qualified with the package name or class name, in one of these formats: - "package package_name" - "class package_name.class_name" - "class package_name.class_name#method_name" If empty, all targets in the module will be run.
    #[serde(rename="testTargets")]
    pub test_targets: Option<Vec<String>>,
}

impl client::Part for AndroidInstrumentationTest {}


/// A set of Android device configuration permutations is defined by the the cross-product of the given axes. Internally, the given AndroidMatrix will be expanded into a set of AndroidDevices. Only supported permutations will be instantiated. Invalid permutations (e.g., incompatible models/versions) are ignored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidMatrix {
    /// Required. The ids of the set of Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidModelIds")]
    pub android_model_ids: Option<Vec<String>>,
    /// Required. The ids of the set of Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="androidVersionIds")]
    pub android_version_ids: Option<Vec<String>>,
    /// Required. The set of locales the test device will enable for testing. Use the TestEnvironmentDiscoveryService to get supported options.
    pub locales: Option<Vec<String>>,
    /// Required. The set of orientations to test with. Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientations: Option<Vec<String>>,
}

impl client::Part for AndroidMatrix {}


/// A description of an Android device tests may be run on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidModel {
    /// The company that this device is branded with. Example: "Google", "Samsung".
    pub brand: Option<String>,
    /// The name of the industrial design. This corresponds to android.os.Build.DEVICE.
    pub codename: Option<String>,
    /// Whether this device is virtual or physical.
    pub form: Option<String>,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// The unique opaque id for this model. Use this for invoking the TestExecutionService.
    pub id: Option<String>,
    /// True if and only if tests with this model are recorded by stitching together screenshots. See use_low_spec_video_recording in device config.
    #[serde(rename="lowFpsVideoRecording")]
    pub low_fps_video_recording: Option<bool>,
    /// The manufacturer of this device.
    pub manufacturer: Option<String>,
    /// The human-readable marketing name for this device model. Examples: "Nexus 5", "Galaxy S5".
    pub name: Option<String>,
    /// Screen density in DPI. This corresponds to ro.sf.lcd_density
    #[serde(rename="screenDensity")]
    pub screen_density: Option<i32>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[serde(rename="screenX")]
    pub screen_x: Option<i32>,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[serde(rename="screenY")]
    pub screen_y: Option<i32>,
    /// The list of supported ABIs for this device. This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2. The most preferred ABI is the first element in the list. Elements are optionally prefixed by "version_id:" (where version_id is the id of an AndroidVersion), denoting an ABI that is supported only on a particular version.
    #[serde(rename="supportedAbis")]
    pub supported_abis: Option<Vec<String>>,
    /// The set of Android versions this device supports.
    #[serde(rename="supportedVersionIds")]
    pub supported_version_ids: Option<Vec<String>>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// URL of a thumbnail image (photo) of the device. e.g. https://lh3.googleusercontent.com/90WcauuJiCYABEl8U0lcZeuS5STUbf2yW...
    #[serde(rename="thumbnailUrl")]
    pub thumbnail_url: Option<String>,
}

impl client::Part for AndroidModel {}


/// A test of an android application that explores the application on a virtual or physical Android Device, finding culprits and crashes as it goes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidRoboTest {
    /// The APK for the application under test.
    #[serde(rename="appApk")]
    pub app_apk: Option<FileReference>,
    /// A multi-apk app bundle for the application under test.
    #[serde(rename="appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The initial activity that should be used to start the app.
    #[serde(rename="appInitialActivity")]
    pub app_initial_activity: Option<String>,
    /// The java package for the application under test. The default value is determined by examining the application's manifest.
    #[serde(rename="appPackageId")]
    pub app_package_id: Option<String>,
    /// A set of directives Robo should apply during the crawl. This allows users to customize the crawl. For example, the username and password for a test account can be provided.
    #[serde(rename="roboDirectives")]
    pub robo_directives: Option<Vec<RoboDirective>>,
    /// The mode in which Robo should run. Most clients should allow the server to populate this field automatically.
    #[serde(rename="roboMode")]
    pub robo_mode: Option<String>,
    /// A JSON file with a sequence of actions Robo should perform as a prologue for the crawl.
    #[serde(rename="roboScript")]
    pub robo_script: Option<FileReference>,
    /// The intents used to launch the app for the crawl. If none are provided, then the main launcher activity is launched. If some are provided, then only those provided are launched (the main launcher activity must be provided explicitly).
    #[serde(rename="startingIntents")]
    pub starting_intents: Option<Vec<RoboStartingIntent>>,
}

impl client::Part for AndroidRoboTest {}


/// Android configuration that can be selected at the time a test is run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidRuntimeConfiguration {
    /// The set of available locales.
    pub locales: Option<Vec<Locale>>,
    /// The set of available orientations.
    pub orientations: Option<Vec<Orientation>>,
}

impl client::Part for AndroidRuntimeConfiguration {}


/// A test of an Android Application with a Test Loop. The intent \ will be implicitly added, since Games is the only user of this api, for the time being.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidTestLoop {
    /// The APK for the application under test.
    #[serde(rename="appApk")]
    pub app_apk: Option<FileReference>,
    /// A multi-apk app bundle for the application under test.
    #[serde(rename="appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The java package for the application under test. The default is determined by examining the application's manifest.
    #[serde(rename="appPackageId")]
    pub app_package_id: Option<String>,
    /// The list of scenario labels that should be run during the test. The scenario labels should map to labels defined in the application's manifest. For example, player_experience and com.google.test.loops.player_experience add all of the loops labeled in the manifest with the com.google.test.loops.player_experience name to the execution. Scenarios can also be specified in the scenarios field.
    #[serde(rename="scenarioLabels")]
    pub scenario_labels: Option<Vec<String>>,
    /// The list of scenarios that should be run during the test. The default is all test loops, derived from the application's manifest.
    pub scenarios: Option<Vec<i32>>,
}

impl client::Part for AndroidTestLoop {}


/// A version of the Android OS.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidVersion {
    /// The API level for this Android version. Examples: 18, 19.
    #[serde(rename="apiLevel")]
    pub api_level: Option<i32>,
    /// The code name for this Android version. Examples: "JellyBean", "KitKat".
    #[serde(rename="codeName")]
    pub code_name: Option<String>,
    /// Market share for this version.
    pub distribution: Option<Distribution>,
    /// An opaque id for this Android version. Use this id to invoke the TestExecutionService.
    pub id: Option<String>,
    /// The date this Android version became available in the market.
    #[serde(rename="releaseDate")]
    pub release_date: Option<Date>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// A string representing this version of the Android OS. Examples: "4.3", "4.4".
    #[serde(rename="versionString")]
    pub version_string: Option<String>,
}

impl client::Part for AndroidVersion {}


/// An Android package file to install.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Apk {
    /// The path to an APK to be installed on the device before the test begins.
    pub location: Option<FileReference>,
    /// The java package for the APK to be installed. Value is determined by examining the application's manifest.
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
}

impl client::Part for Apk {}


/// Android application details based on application manifest and apk archive contents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkDetail {
    /// no description provided
    #[serde(rename="apkManifest")]
    pub apk_manifest: Option<ApkManifest>,
}

impl client::Part for ApkDetail {}


/// An Android app manifest. See http://developer.android.com/guide/topics/manifest/manifest-intro.html
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApkManifest {
    /// User-readable name for the application.
    #[serde(rename="applicationLabel")]
    pub application_label: Option<String>,
    /// no description provided
    #[serde(rename="intentFilters")]
    pub intent_filters: Option<Vec<IntentFilter>>,
    /// Maximum API level on which the application is designed to run.
    #[serde(rename="maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
    /// Minimum API level required for the application to run.
    #[serde(rename="minSdkVersion")]
    pub min_sdk_version: Option<i32>,
    /// Full Java-style package name for this application, e.g. "com.example.foo".
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
    /// Specifies the API Level on which the application is designed to run.
    #[serde(rename="targetSdkVersion")]
    pub target_sdk_version: Option<i32>,
    /// Permissions declared to be used by the application
    #[serde(rename="usesPermission")]
    pub uses_permission: Option<Vec<String>>,
}

impl client::Part for ApkManifest {}


/// An Android App Bundle file format, containing a BundleConfig.pb file, a base module directory, zero or more dynamic feature module directories. See https://developer.android.com/guide/app-bundle/build for guidance on building App Bundles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppBundle {
    /// .aab file representing the app bundle under test.
    #[serde(rename="bundleLocation")]
    pub bundle_location: Option<FileReference>,
}

impl client::Part for AppBundle {}


/// Response containing the current state of the specified test matrix.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test matrices cancel projects](ProjectTestMatriceCancelCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelTestMatrixResponse {
    /// The current rolled-up state of the test matrix. If this state is already final, then the cancelation request will have no effect.
    #[serde(rename="testState")]
    pub test_state: Option<String>,
}

impl client::ResponseResult for CancelTestMatrixResponse {}


/// Information about the client which invoked the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    /// The list of detailed information about client.
    #[serde(rename="clientInfoDetails")]
    pub client_info_details: Option<Vec<ClientInfoDetail>>,
    /// Required. Client name, such as gcloud.
    pub name: Option<String>,
}

impl client::Part for ClientInfo {}


/// Key-value pair of detailed information about the client which invoked the test. Examples: {'Version', '1.0'}, {'Release Track', 'BETA'}.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfoDetail {
    /// Required. The key of detailed client information.
    pub key: Option<String>,
    /// Required. The value of detailed client information.
    pub value: Option<String>,
}

impl client::Part for ClientInfoDetail {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day, with a zero year (e.g., an anniversary) * A year on its own, with a zero month and a zero day * A year and month, with a zero day (e.g., a credit card expiration date) Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// A single device file description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceFile {
    /// A reference to an opaque binary blob file.
    #[serde(rename="obbFile")]
    pub obb_file: Option<ObbFile>,
    /// A reference to a regular file.
    #[serde(rename="regularFile")]
    pub regular_file: Option<RegularFile>,
}

impl client::Part for DeviceFile {}


/// A single device IP block
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceIpBlock {
    /// The date this block was added to Firebase Test Lab
    #[serde(rename="addedDate")]
    pub added_date: Option<Date>,
    /// An IP address block in CIDR notation eg: 34.68.194.64/29
    pub block: Option<String>,
    /// Whether this block is used by physical or virtual devices
    pub form: Option<String>,
}

impl client::Part for DeviceIpBlock {}


/// List of IP blocks used by the Firebase Test Lab
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceIpBlockCatalog {
    /// The device IP blocks used by Firebase Test Lab
    #[serde(rename="ipBlocks")]
    pub ip_blocks: Option<Vec<DeviceIpBlock>>,
}

impl client::Part for DeviceIpBlockCatalog {}


/// Data about the relative number of devices running a given configuration of the Android platform.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    /// Output only. The estimated fraction (0-1) of the total market with this configuration.
    #[serde(rename="marketShare")]
    pub market_share: Option<f64>,
    /// Output only. The time this distribution was measured.
    #[serde(rename="measurementTime")]
    pub measurement_time: Option<String>,
}

impl client::Part for Distribution {}


/// The environment in which the test is run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// An Android device which must be used with an Android test.
    #[serde(rename="androidDevice")]
    pub android_device: Option<AndroidDevice>,
    /// An iOS device which must be used with an iOS test.
    #[serde(rename="iosDevice")]
    pub ios_device: Option<IosDevice>,
}

impl client::Part for Environment {}


/// The matrix of environments in which the test is to be executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentMatrix {
    /// A list of Android devices; the test will be run only on the specified devices.
    #[serde(rename="androidDeviceList")]
    pub android_device_list: Option<AndroidDeviceList>,
    /// A matrix of Android devices.
    #[serde(rename="androidMatrix")]
    pub android_matrix: Option<AndroidMatrix>,
    /// A list of iOS devices.
    #[serde(rename="iosDeviceList")]
    pub ios_device_list: Option<IosDeviceList>,
}

impl client::Part for EnvironmentMatrix {}


/// A key-value pair passed as an environment variable to the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// Key for the environment variable.
    pub key: Option<String>,
    /// Value for the environment variable.
    pub value: Option<String>,
}

impl client::Part for EnvironmentVariable {}


/// A reference to a file, used for user inputs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get apk details application detail service](ApplicationDetailServiceGetApkDetailCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileReference {
    /// A path to a file in Google Cloud Storage. Example: gs://build-app-1414623860166/app%40debug-unaligned.apk These paths are expected to be url encoded (percent encoding)
    #[serde(rename="gcsPath")]
    pub gcs_path: Option<String>,
}

impl client::RequestValue for FileReference {}


/// Response containing the details of the specified Android application APK.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get apk details application detail service](ApplicationDetailServiceGetApkDetailCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetApkDetailsResponse {
    /// Details of the Android APK.
    #[serde(rename="apkDetail")]
    pub apk_detail: Option<ApkDetail>,
}

impl client::ResponseResult for GetApkDetailsResponse {}


/// Enables automatic Google account login. If set, the service automatically generates a Google test account and adds it to the device, before executing the test. Note that test accounts might be reused. Many applications show their full set of functionalities when an account is present on the device. Logging into the device with these generated accounts allows testing more functionalities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAuto { _never_set: Option<bool> }

impl client::Part for GoogleAuto {}


/// A storage location within Google cloud storage (GCS).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudStorage {
    /// Required. The path to a directory in GCS that will eventually contain the results for this test. The requesting user must have write access on the bucket in the supplied path.
    #[serde(rename="gcsPath")]
    pub gcs_path: Option<String>,
}

impl client::Part for GoogleCloudStorage {}


/// The section of an tag. https://developer.android.com/guide/topics/manifest/intent-filter-element.html
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntentFilter {
    /// The android:name value of the tag.
    #[serde(rename="actionNames")]
    pub action_names: Option<Vec<String>>,
    /// The android:name value of the tag.
    #[serde(rename="categoryNames")]
    pub category_names: Option<Vec<String>>,
    /// The android:mimeType value of the tag.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
}

impl client::Part for IntentFilter {}


/// A single iOS device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDevice {
    /// Required. The id of the iOS device to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="iosModelId")]
    pub ios_model_id: Option<String>,
    /// Required. The id of the iOS major software version to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename="iosVersionId")]
    pub ios_version_id: Option<String>,
    /// Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options.
    pub locale: Option<String>,
    /// Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientation: Option<String>,
}

impl client::Part for IosDevice {}


/// The currently supported iOS devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDeviceCatalog {
    /// The set of supported iOS device models.
    pub models: Option<Vec<IosModel>>,
    /// The set of supported runtime configurations.
    #[serde(rename="runtimeConfiguration")]
    pub runtime_configuration: Option<IosRuntimeConfiguration>,
    /// The set of supported iOS software versions.
    pub versions: Option<Vec<IosVersion>>,
    /// The set of supported Xcode versions.
    #[serde(rename="xcodeVersions")]
    pub xcode_versions: Option<Vec<XcodeVersion>>,
}

impl client::Part for IosDeviceCatalog {}


/// A file or directory to install on the device before the test starts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDeviceFile {
    /// The bundle id of the app where this file lives. iOS apps sandbox their own filesystem, so app files must specify which app installed on the device.
    #[serde(rename="bundleId")]
    pub bundle_id: Option<String>,
    /// The source file
    pub content: Option<FileReference>,
    /// Location of the file on the device, inside the app's sandboxed filesystem
    #[serde(rename="devicePath")]
    pub device_path: Option<String>,
}

impl client::Part for IosDeviceFile {}


/// A list of iOS device configurations in which the test is to be executed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosDeviceList {
    /// Required. A list of iOS devices.
    #[serde(rename="iosDevices")]
    pub ios_devices: Option<Vec<IosDevice>>,
}

impl client::Part for IosDeviceList {}


/// A description of an iOS device tests may be run on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosModel {
    /// Device capabilities. Copied from https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html
    #[serde(rename="deviceCapabilities")]
    pub device_capabilities: Option<Vec<String>>,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// The unique opaque id for this model. Use this for invoking the TestExecutionService.
    pub id: Option<String>,
    /// The human-readable name for this device model. Examples: "iPhone 4s", "iPad Mini 2".
    pub name: Option<String>,
    /// Screen density in DPI.
    #[serde(rename="screenDensity")]
    pub screen_density: Option<i32>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[serde(rename="screenX")]
    pub screen_x: Option<i32>,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[serde(rename="screenY")]
    pub screen_y: Option<i32>,
    /// The set of iOS major software versions this device supports.
    #[serde(rename="supportedVersionIds")]
    pub supported_version_ids: Option<Vec<String>>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
}

impl client::Part for IosModel {}


/// iOS configuration that can be selected at the time a test is run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosRuntimeConfiguration {
    /// The set of available locales.
    pub locales: Option<Vec<Locale>>,
    /// The set of available orientations.
    pub orientations: Option<Vec<Orientation>>,
}

impl client::Part for IosRuntimeConfiguration {}


/// A test of an iOS application that implements one or more game loop scenarios. This test type accepts an archived application (.ipa file) and a list of integer scenarios that will be executed on the app sequentially.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosTestLoop {
    /// Output only. The bundle id for the application under test.
    #[serde(rename="appBundleId")]
    pub app_bundle_id: Option<String>,
    /// Required. The .ipa of the application to test.
    #[serde(rename="appIpa")]
    pub app_ipa: Option<FileReference>,
    /// The list of scenarios that should be run during the test. Defaults to the single scenario 0 if unspecified.
    pub scenarios: Option<Vec<i32>>,
}

impl client::Part for IosTestLoop {}


/// A description of how to set up an iOS device prior to running the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosTestSetup {
    /// iOS apps to install in addition to those being directly tested.
    #[serde(rename="additionalIpas")]
    pub additional_ipas: Option<Vec<FileReference>>,
    /// The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[serde(rename="networkProfile")]
    pub network_profile: Option<String>,
    /// List of directories on the device to upload to Cloud Storage at the end of the test. Directories should either be in a shared directory (such as /private/var/mobile/Media) or within an accessible directory inside the app's filesystem (such as /Documents) by specifying the bundle ID.
    #[serde(rename="pullDirectories")]
    pub pull_directories: Option<Vec<IosDeviceFile>>,
    /// List of files to push to the device before starting the test.
    #[serde(rename="pushFiles")]
    pub push_files: Option<Vec<IosDeviceFile>>,
}

impl client::Part for IosTestSetup {}


/// An iOS version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosVersion {
    /// An opaque id for this iOS version. Use this id to invoke the TestExecutionService.
    pub id: Option<String>,
    /// An integer representing the major iOS version. Examples: "8", "9".
    #[serde(rename="majorVersion")]
    pub major_version: Option<i32>,
    /// An integer representing the minor iOS version. Examples: "1", "2".
    #[serde(rename="minorVersion")]
    pub minor_version: Option<i32>,
    /// The available Xcode versions for this version.
    #[serde(rename="supportedXcodeVersionIds")]
    pub supported_xcode_version_ids: Option<Vec<String>>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
}

impl client::Part for IosVersion {}


/// A test of an iOS application that uses the XCTest framework. Xcode supports the option to "build for testing", which generates an .xctestrun file that contains a test specification (arguments, test methods, etc). This test type accepts a zip file containing the .xctestrun file and the corresponding contents of the Build/Products directory that contains all the binaries needed to run the tests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosXcTest {
    /// Output only. The bundle id for the application under test.
    #[serde(rename="appBundleId")]
    pub app_bundle_id: Option<String>,
    /// The option to test special app entitlements. Setting this would re-sign the app having special entitlements with an explicit application-identifier. Currently supports testing aps-environment entitlement.
    #[serde(rename="testSpecialEntitlements")]
    pub test_special_entitlements: Option<bool>,
    /// Required. The .zip containing the .xctestrun file and the contents of the DerivedData/Build/Products directory. The .xctestrun file in this zip is ignored if the xctestrun field is specified.
    #[serde(rename="testsZip")]
    pub tests_zip: Option<FileReference>,
    /// The Xcode version that should be used for the test. Use the TestEnvironmentDiscoveryService to get supported options. Defaults to the latest Xcode version Firebase Test Lab supports.
    #[serde(rename="xcodeVersion")]
    pub xcode_version: Option<String>,
    /// An .xctestrun file that will override the .xctestrun file in the tests zip. Because the .xctestrun file contains environment variables along with test methods to run and/or ignore, this can be useful for sharding tests. Default is taken from the tests zip.
    pub xctestrun: Option<FileReference>,
}

impl client::Part for IosXcTest {}


/// Specifies an intent that starts the main launcher activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LauncherActivityIntent { _never_set: Option<bool> }

impl client::Part for LauncherActivityIntent {}


/// A location/region designation for language.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Locale {
    /// The id for this locale. Example: "en_US".
    pub id: Option<String>,
    /// A human-friendly name for this language/locale. Example: "English".
    pub name: Option<String>,
    /// A human-friendly string representing the region for this locale. Example: "United States". Not present for every locale.
    pub region: Option<String>,
    /// Tags for this dimension. Example: "default".
    pub tags: Option<Vec<String>>,
}

impl client::Part for Locale {}


/// Shards test cases into the specified groups of packages, classes, and/or methods. With manual sharding enabled, specifying test targets via environment_variables or in InstrumentationTest is invalid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualSharding {
    /// Required. Group of packages, classes, and/or test methods to be run for each shard. When any physical devices are selected, the number of test_targets_for_shard must be >= 1 and <= 50. When no physical devices are selected, the number must be >= 1 and <= 500.
    #[serde(rename="testTargetsForShard")]
    pub test_targets_for_shard: Option<Vec<TestTargetsForShard>>,
}

impl client::Part for ManualSharding {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    /// The emulation rule applying to the download traffic.
    #[serde(rename="downRule")]
    pub down_rule: Option<TrafficRule>,
    /// The unique opaque id for this network traffic configuration.
    pub id: Option<String>,
    /// The emulation rule applying to the upload traffic.
    #[serde(rename="upRule")]
    pub up_rule: Option<TrafficRule>,
}

impl client::Part for NetworkConfiguration {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfigurationCatalog {
    /// no description provided
    pub configurations: Option<Vec<NetworkConfiguration>>,
}

impl client::Part for NetworkConfigurationCatalog {}


/// An opaque binary blob file to install on the device before the test starts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObbFile {
    /// Required. Opaque Binary Blob (OBB) file(s) to install on the device.
    pub obb: Option<FileReference>,
    /// Required. OBB file name which must conform to the format as specified by Android e.g. [main|patch].0300110.com.example.android.obb which will be installed into \/Android/obb/\/ on the device.
    #[serde(rename="obbFileName")]
    pub obb_file_name: Option<String>,
}

impl client::Part for ObbFile {}


/// Screen orientation of the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Orientation {
    /// The id for this orientation. Example: "portrait".
    pub id: Option<String>,
    /// A human-friendly name for this orientation. Example: "portrait".
    pub name: Option<String>,
    /// Tags for this dimension. Example: "default".
    pub tags: Option<Vec<String>>,
}

impl client::Part for Orientation {}


/// The currently provided software environment on the devices under test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProvidedSoftwareCatalog {
    /// A string representing the current version of AndroidX Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#androidx.test:orchestrator.
    #[serde(rename="androidxOrchestratorVersion")]
    pub androidx_orchestrator_version: Option<String>,
    /// Deprecated: Use AndroidX Test Orchestrator going forward. A string representing the current version of Android Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#com.android.support.test:orchestrator.
    #[serde(rename="orchestratorVersion")]
    pub orchestrator_version: Option<String>,
}

impl client::Part for ProvidedSoftwareCatalog {}


/// A file or directory to install on the device before the test starts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegularFile {
    /// Required. The source file.
    pub content: Option<FileReference>,
    /// Required. Where to put the content on the device. Must be an absolute, allowlisted path. If the file exists, it will be replaced. The following device-side directories and any of their subdirectories are allowlisted: ${EXTERNAL_STORAGE}, /sdcard, or /storage ${ANDROID_DATA}/local/tmp, or /data/local/tmp Specifying a path outside of these directory trees is invalid. The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device and copy the file there. It is strongly advised to use the Environment API in app and test code to access files on the device in a portable way.
    #[serde(rename="devicePath")]
    pub device_path: Option<String>,
}

impl client::Part for RegularFile {}


/// Locations where the results of running the test are stored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultStorage {
    /// Required.
    #[serde(rename="googleCloudStorage")]
    pub google_cloud_storage: Option<GoogleCloudStorage>,
    /// Output only. URL to the results in the Firebase Web Console.
    #[serde(rename="resultsUrl")]
    pub results_url: Option<String>,
    /// Output only. The tool results execution that results are written to.
    #[serde(rename="toolResultsExecution")]
    pub tool_results_execution: Option<ToolResultsExecution>,
    /// The tool results history that contains the tool results execution that results are written to. If not provided, the service will choose an appropriate value.
    #[serde(rename="toolResultsHistory")]
    pub tool_results_history: Option<ToolResultsHistory>,
}

impl client::Part for ResultStorage {}


/// Directs Robo to interact with a specific UI element if it is encountered during the crawl. Currently, Robo can perform text entry or element click.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoboDirective {
    /// Required. The type of action that Robo should perform on the specified element.
    #[serde(rename="actionType")]
    pub action_type: Option<String>,
    /// The text that Robo is directed to set. If left empty, the directive will be treated as a CLICK on the element matching the resource_name.
    #[serde(rename="inputText")]
    pub input_text: Option<String>,
    /// Required. The android resource name of the target UI element. For example, in Java: R.string.foo in xml: @string/foo Only the "foo" part is needed. Reference doc: https://developer.android.com/guide/topics/resources/accessing-resources.html
    #[serde(rename="resourceName")]
    pub resource_name: Option<String>,
}

impl client::Part for RoboDirective {}


/// Message for specifying the start activities to crawl.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoboStartingIntent {
    /// An intent that starts the main launcher activity.
    #[serde(rename="launcherActivity")]
    pub launcher_activity: Option<LauncherActivityIntent>,
    /// An intent that starts an activity with specific details.
    #[serde(rename="startActivity")]
    pub start_activity: Option<StartActivityIntent>,
    /// Timeout in seconds for each intent.
    pub timeout: Option<String>,
}

impl client::Part for RoboStartingIntent {}


/// Output only. Details about the shard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Shard {
    /// Output only. The total number of shards.
    #[serde(rename="numShards")]
    pub num_shards: Option<i32>,
    /// Output only. The index of the shard among all the shards.
    #[serde(rename="shardIndex")]
    pub shard_index: Option<i32>,
    /// Output only. Test targets for each shard.
    #[serde(rename="testTargetsForShard")]
    pub test_targets_for_shard: Option<TestTargetsForShard>,
}

impl client::Part for Shard {}


/// Options for enabling sharding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShardingOption {
    /// Shards test cases into the specified groups of packages, classes, and/or methods.
    #[serde(rename="manualSharding")]
    pub manual_sharding: Option<ManualSharding>,
    /// Uniformly shards test cases given a total number of shards.
    #[serde(rename="uniformSharding")]
    pub uniform_sharding: Option<UniformSharding>,
}

impl client::Part for ShardingOption {}


/// A starting intent specified by an action, uri, and categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartActivityIntent {
    /// Action name. Required for START_ACTIVITY.
    pub action: Option<String>,
    /// Intent categories to set on the intent.
    pub categories: Option<Vec<String>>,
    /// URI for the action.
    pub uri: Option<String>,
}

impl client::Part for StartActivityIntent {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystraceSetup {
    /// Systrace duration in seconds. Should be between 1 and 30 seconds. 0 disables systrace.
    #[serde(rename="durationSeconds")]
    pub duration_seconds: Option<i32>,
}

impl client::Part for SystraceSetup {}


/// Additional details about the progress of the running test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestDetails {
    /// Output only. If the TestState is ERROR, then this string will contain human-readable details about the error.
    #[serde(rename="errorMessage")]
    pub error_message: Option<String>,
    /// Output only. Human-readable, detailed descriptions of the test's progress. For example: "Provisioning a device", "Starting Test". During the course of execution new data may be appended to the end of progress_messages.
    #[serde(rename="progressMessages")]
    pub progress_messages: Option<Vec<String>>,
}

impl client::Part for TestDetails {}


/// A description of a test environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get test environment catalog](TestEnvironmentCatalogGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestEnvironmentCatalog {
    /// Supported Android devices.
    #[serde(rename="androidDeviceCatalog")]
    pub android_device_catalog: Option<AndroidDeviceCatalog>,
    /// The IP blocks used by devices in the test environment.
    #[serde(rename="deviceIpBlockCatalog")]
    pub device_ip_block_catalog: Option<DeviceIpBlockCatalog>,
    /// Supported iOS devices.
    #[serde(rename="iosDeviceCatalog")]
    pub ios_device_catalog: Option<IosDeviceCatalog>,
    /// Supported network configurations.
    #[serde(rename="networkConfigurationCatalog")]
    pub network_configuration_catalog: Option<NetworkConfigurationCatalog>,
    /// The software test environment provided by TestExecutionService.
    #[serde(rename="softwareCatalog")]
    pub software_catalog: Option<ProvidedSoftwareCatalog>,
}

impl client::ResponseResult for TestEnvironmentCatalog {}


/// A single test executed in a single environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestExecution {
    /// Output only. How the host machine(s) are configured.
    pub environment: Option<Environment>,
    /// Output only. Unique id set by the service.
    pub id: Option<String>,
    /// Output only. Id of the containing TestMatrix.
    #[serde(rename="matrixId")]
    pub matrix_id: Option<String>,
    /// Output only. The cloud project that owns the test execution.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Output only. Details about the shard.
    pub shard: Option<Shard>,
    /// Output only. Indicates the current progress of the test execution (e.g., FINISHED).
    pub state: Option<String>,
    /// Output only. Additional details about the running test.
    #[serde(rename="testDetails")]
    pub test_details: Option<TestDetails>,
    /// Output only. How to run the test.
    #[serde(rename="testSpecification")]
    pub test_specification: Option<TestSpecification>,
    /// Output only. The time this test execution was initially created.
    pub timestamp: Option<String>,
    /// Output only. Where the results for this execution are written.
    #[serde(rename="toolResultsStep")]
    pub tool_results_step: Option<ToolResultsStep>,
}

impl client::Part for TestExecution {}


/// TestMatrix captures all details about a test. It contains the environment configuration, test specification, test executions and overall state and outcome.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test matrices create projects](ProjectTestMatriceCreateCall) (request|response)
/// * [test matrices get projects](ProjectTestMatriceGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestMatrix {
    /// Information about the client which invoked the test.
    #[serde(rename="clientInfo")]
    pub client_info: Option<ClientInfo>,
    /// Required. The devices the tests are being executed on.
    #[serde(rename="environmentMatrix")]
    pub environment_matrix: Option<EnvironmentMatrix>,
    /// If true, only a single attempt at most will be made to run each execution/shard in the matrix. Flaky test attempts are not affected. Normally, 2 or more attempts are made if a potential infrastructure issue is detected. This feature is for latency sensitive workloads. The incidence of execution failures may be significantly greater for fail-fast matrices and support is more limited because of that expectation.
    #[serde(rename="failFast")]
    pub fail_fast: Option<bool>,
    /// The number of times a TestExecution should be re-attempted if one or more of its test cases fail for any reason. The maximum number of reruns allowed is 10. Default is 0, which implies no reruns.
    #[serde(rename="flakyTestAttempts")]
    pub flaky_test_attempts: Option<i32>,
    /// Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state.
    #[serde(rename="invalidMatrixDetails")]
    pub invalid_matrix_details: Option<String>,
    /// Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED.
    #[serde(rename="outcomeSummary")]
    pub outcome_summary: Option<String>,
    /// The cloud project that owns the test matrix.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Required. Where the results for the matrix are written.
    #[serde(rename="resultStorage")]
    pub result_storage: Option<ResultStorage>,
    /// Output only. Indicates the current progress of the test matrix.
    pub state: Option<String>,
    /// Output only. The list of test executions that the service creates for this matrix.
    #[serde(rename="testExecutions")]
    pub test_executions: Option<Vec<TestExecution>>,
    /// Output only. Unique id set by the service.
    #[serde(rename="testMatrixId")]
    pub test_matrix_id: Option<String>,
    /// Required. How to run the test.
    #[serde(rename="testSpecification")]
    pub test_specification: Option<TestSpecification>,
    /// Output only. The time this test matrix was initially created.
    pub timestamp: Option<String>,
}

impl client::RequestValue for TestMatrix {}
impl client::ResponseResult for TestMatrix {}


/// A description of how to set up the Android device prior to running the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestSetup {
    /// The device will be logged in on this account for the duration of the test.
    pub account: Option<Account>,
    /// APKs to install in addition to those being directly tested. Currently capped at 100.
    #[serde(rename="additionalApks")]
    pub additional_apks: Option<Vec<Apk>>,
    /// List of directories on the device to upload to GCS at the end of the test; they must be absolute paths under /sdcard, /storage or /data/local/tmp. Path names are restricted to characters a-z A-Z 0-9 _ - . + and / Note: The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device.
    #[serde(rename="directoriesToPull")]
    pub directories_to_pull: Option<Vec<String>>,
    /// Whether to prevent all runtime permissions to be granted at app install
    #[serde(rename="dontAutograntPermissions")]
    pub dont_autogrant_permissions: Option<bool>,
    /// Environment variables to set for the test (only applicable for instrumentation tests).
    #[serde(rename="environmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    /// List of files to push to the device before starting the test.
    #[serde(rename="filesToPush")]
    pub files_to_push: Option<Vec<DeviceFile>>,
    /// The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[serde(rename="networkProfile")]
    pub network_profile: Option<String>,
    /// Deprecated: Systrace uses Python 2 which has been sunset 2020-01-01. Support of Systrace may stop at any time, at which point no Systrace file will be provided in the results. Systrace configuration for the run. If set a systrace will be taken, starting on test start and lasting for the configured duration. The systrace file thus obtained is put in the results bucket together with the other artifacts from the run.
    pub systrace: Option<SystraceSetup>,
}

impl client::Part for TestSetup {}


/// A description of how to run the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestSpecification {
    /// An Android instrumentation test.
    #[serde(rename="androidInstrumentationTest")]
    pub android_instrumentation_test: Option<AndroidInstrumentationTest>,
    /// An Android robo test.
    #[serde(rename="androidRoboTest")]
    pub android_robo_test: Option<AndroidRoboTest>,
    /// An Android Application with a Test Loop.
    #[serde(rename="androidTestLoop")]
    pub android_test_loop: Option<AndroidTestLoop>,
    /// Disables performance metrics recording. May reduce test latency.
    #[serde(rename="disablePerformanceMetrics")]
    pub disable_performance_metrics: Option<bool>,
    /// Disables video recording. May reduce test latency.
    #[serde(rename="disableVideoRecording")]
    pub disable_video_recording: Option<bool>,
    /// An iOS application with a test loop.
    #[serde(rename="iosTestLoop")]
    pub ios_test_loop: Option<IosTestLoop>,
    /// Test setup requirements for iOS.
    #[serde(rename="iosTestSetup")]
    pub ios_test_setup: Option<IosTestSetup>,
    /// An iOS XCTest, via an .xctestrun file.
    #[serde(rename="iosXcTest")]
    pub ios_xc_test: Option<IosXcTest>,
    /// Test setup requirements for Android e.g. files to install, bootstrap scripts.
    #[serde(rename="testSetup")]
    pub test_setup: Option<TestSetup>,
    /// Max time a test execution is allowed to run before it is automatically cancelled. The default value is 5 min.
    #[serde(rename="testTimeout")]
    pub test_timeout: Option<String>,
}

impl client::Part for TestSpecification {}


/// Test targets for a shard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestTargetsForShard {
    /// Group of packages, classes, and/or test methods to be run for each shard. The targets need to be specified in AndroidJUnitRunner argument format. For example, "package com.my.packages" "class com.my.package.MyClass". The number of shard_test_targets must be greater than 0.
    #[serde(rename="testTargets")]
    pub test_targets: Option<Vec<String>>,
}

impl client::Part for TestTargetsForShard {}


/// Represents a tool results execution resource. This has the results of a TestMatrix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultsExecution {
    /// Output only. A tool results execution ID.
    #[serde(rename="executionId")]
    pub execution_id: Option<String>,
    /// Output only. A tool results history ID.
    #[serde(rename="historyId")]
    pub history_id: Option<String>,
    /// Output only. The cloud project that owns the tool results execution.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
}

impl client::Part for ToolResultsExecution {}


/// Represents a tool results history resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultsHistory {
    /// Required. A tool results history ID.
    #[serde(rename="historyId")]
    pub history_id: Option<String>,
    /// Required. The cloud project that owns the tool results history.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
}

impl client::Part for ToolResultsHistory {}


/// Represents a tool results step resource. This has the results of a TestExecution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultsStep {
    /// Output only. A tool results execution ID.
    #[serde(rename="executionId")]
    pub execution_id: Option<String>,
    /// Output only. A tool results history ID.
    #[serde(rename="historyId")]
    pub history_id: Option<String>,
    /// Output only. The cloud project that owns the tool results step.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// Output only. A tool results step ID.
    #[serde(rename="stepId")]
    pub step_id: Option<String>,
}

impl client::Part for ToolResultsStep {}


/// Network emulation parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficRule {
    /// Bandwidth in kbits/second.
    pub bandwidth: Option<f32>,
    /// Burst size in kbits.
    pub burst: Option<f32>,
    /// Packet delay, must be >= 0.
    pub delay: Option<String>,
    /// Packet duplication ratio (0.0 - 1.0).
    #[serde(rename="packetDuplicationRatio")]
    pub packet_duplication_ratio: Option<f32>,
    /// Packet loss ratio (0.0 - 1.0).
    #[serde(rename="packetLossRatio")]
    pub packet_loss_ratio: Option<f32>,
}

impl client::Part for TrafficRule {}


/// Uniformly shards test cases given a total number of shards. For Instrumentation test, it will be translated to "-e numShard" "-e shardIndex" AndroidJUnitRunner arguments. Based on the sharding mechanism AndroidJUnitRunner uses, there is no guarantee that test cases will be distributed uniformly across all shards. With uniform sharding enabled, specifying these sharding arguments via environment_variables is invalid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UniformSharding {
    /// Required. Total number of shards. When any physical devices are selected, the number must be >= 1 and <= 50. When no physical devices are selected, the number must be >= 1 and <= 500.
    #[serde(rename="numShards")]
    pub num_shards: Option<i32>,
}

impl client::Part for UniformSharding {}


/// An Xcode version that an iOS version is compatible with.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct XcodeVersion {
    /// Tags for this Xcode version. Example: "default".
    pub tags: Option<Vec<String>>,
    /// The id for this version. Example: "9.2".
    pub version: Option<String>,
}

impl client::Part for XcodeVersion {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *applicationDetailService* resources.
/// It is not used directly, but through the `Testing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_apk_details(...)`
/// // to build up your call.
/// let rb = hub.application_detail_service();
/// # }
/// ```
pub struct ApplicationDetailServiceMethods<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for ApplicationDetailServiceMethods<'a, S> {}

impl<'a, S> ApplicationDetailServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of an Android application APK.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_apk_details(&self, request: FileReference) -> ApplicationDetailServiceGetApkDetailCall<'a, S> {
        ApplicationDetailServiceGetApkDetailCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Testing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `test_matrices_cancel(...)`, `test_matrices_create(...)` and `test_matrices_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test.
    /// * `testMatrixId` - Test matrix that will be canceled.
    pub fn test_matrices_cancel(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceCancelCall<'a, S> {
        ProjectTestMatriceCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The GCE project under which this job will run.
    pub fn test_matrices_create(&self, request: TestMatrix, project_id: &str) -> ProjectTestMatriceCreateCall<'a, S> {
        ProjectTestMatriceCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks the status of a test matrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test matrix.
    /// * `testMatrixId` - Unique test matrix id which was assigned by the service.
    pub fn test_matrices_get(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceGetCall<'a, S> {
        ProjectTestMatriceGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *testEnvironmentCatalog* resources.
/// It is not used directly, but through the `Testing` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.test_environment_catalog();
/// # }
/// ```
pub struct TestEnvironmentCatalogMethods<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for TestEnvironmentCatalogMethods<'a, S> {}

impl<'a, S> TestEnvironmentCatalogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred
    /// 
    /// # Arguments
    ///
    /// * `environmentType` - Required. The type of environment that should be listed.
    pub fn get(&self, environment_type: &str) -> TestEnvironmentCatalogGetCall<'a, S> {
        TestEnvironmentCatalogGetCall {
            hub: self.hub,
            _environment_type: environment_type.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the details of an Android application APK.
///
/// A builder for the *getApkDetails* method supported by a *applicationDetailService* resource.
/// It is not used directly, but through a `ApplicationDetailServiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::FileReference;
/// # async fn dox() {
/// # use std::default::Default;
/// # use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = FileReference::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.application_detail_service().get_apk_details(req)
///              .doit().await;
/// # }
/// ```
pub struct ApplicationDetailServiceGetApkDetailCall<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
    _request: FileReference,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ApplicationDetailServiceGetApkDetailCall<'a, S> {}

impl<'a, S> ApplicationDetailServiceGetApkDetailCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetApkDetailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "testing.applicationDetailService.getApkDetails",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/applicationDetailService/getApkDetails";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: FileReference) -> ApplicationDetailServiceGetApkDetailCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ApplicationDetailServiceGetApkDetailCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ApplicationDetailServiceGetApkDetailCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ApplicationDetailServiceGetApkDetailCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
///
/// A builder for the *testMatrices.cancel* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_cancel("projectId", "testMatrixId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectTestMatriceCancelCall<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
    _project_id: String,
    _test_matrix_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ProjectTestMatriceCancelCall<'a, S> {}

impl<'a, S> ProjectTestMatriceCancelCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CancelTestMatrixResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "testing.projects.testMatrices.cancel",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("testMatrixId", self._test_matrix_id.to_string()));
        for &field in ["alt", "projectId", "testMatrixId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices/{testMatrixId}:cancel";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{testMatrixId}", "testMatrixId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["testMatrixId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Cloud project that owns the test.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceCancelCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Test matrix that will be canceled.
    ///
    /// Sets the *test matrix id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn test_matrix_id(mut self, new_value: &str) -> ProjectTestMatriceCancelCall<'a, S> {
        self._test_matrix_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectTestMatriceCancelCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceCancelCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ProjectTestMatriceCancelCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.
///
/// A builder for the *testMatrices.create* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::TestMatrix;
/// # async fn dox() {
/// # use std::default::Default;
/// # use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestMatrix::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_create(req, "projectId")
///              .request_id("takimata")
///              .doit().await;
/// # }
/// ```
pub struct ProjectTestMatriceCreateCall<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
    _request: TestMatrix,
    _project_id: String,
    _request_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ProjectTestMatriceCreateCall<'a, S> {}

impl<'a, S> ProjectTestMatriceCreateCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestMatrix)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "testing.projects.testMatrices.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        if let Some(value) = self._request_id {
            params.push(("requestId", value.to_string()));
        }
        for &field in ["alt", "projectId", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: TestMatrix) -> ProjectTestMatriceCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The GCE project under which this job will run.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceCreateCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// A string id used to detect duplicated requests. Ids are automatically scoped to a project, so users should ensure the ID is unique per-project. A UUID is recommended. Optional, but strongly recommended.
    ///
    /// Sets the *request id* query property to the given value.
    pub fn request_id(mut self, new_value: &str) -> ProjectTestMatriceCreateCall<'a, S> {
        self._request_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectTestMatriceCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ProjectTestMatriceCreateCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Checks the status of a test matrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
///
/// A builder for the *testMatrices.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_get("projectId", "testMatrixId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectTestMatriceGetCall<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
    _project_id: String,
    _test_matrix_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for ProjectTestMatriceGetCall<'a, S> {}

impl<'a, S> ProjectTestMatriceGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestMatrix)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "testing.projects.testMatrices.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("testMatrixId", self._test_matrix_id.to_string()));
        for &field in ["alt", "projectId", "testMatrixId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices/{testMatrixId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{testMatrixId}", "testMatrixId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["testMatrixId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Cloud project that owns the test matrix.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceGetCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Unique test matrix id which was assigned by the service.
    ///
    /// Sets the *test matrix id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn test_matrix_id(mut self, new_value: &str) -> ProjectTestMatriceGetCall<'a, S> {
        self._test_matrix_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectTestMatriceGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> ProjectTestMatriceGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred
///
/// A builder for the *get* method supported by a *testEnvironmentCatalog* resource.
/// It is not used directly, but through a `TestEnvironmentCatalogMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use testing1::{Testing, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.test_environment_catalog().get("environmentType")
///              .project_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct TestEnvironmentCatalogGetCall<'a, S>
    where S: 'a {

    hub: &'a Testing<S>,
    _environment_type: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, S> client::CallBuilder for TestEnvironmentCatalogGetCall<'a, S> {}

impl<'a, S> TestEnvironmentCatalogGetCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestEnvironmentCatalog)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "testing.testEnvironmentCatalog.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("environmentType", self._environment_type.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "environmentType", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/testEnvironmentCatalog/{environmentType}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{environmentType}", "environmentType")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["environmentType"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The type of environment that should be listed.
    ///
    /// Sets the *environment type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn environment_type(mut self, new_value: &str) -> TestEnvironmentCatalogGetCall<'a, S> {
        self._environment_type = new_value.to_string();
        self
    }
    /// For authorization, the cloud project requesting the TestEnvironmentCatalog.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> TestEnvironmentCatalogGetCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TestEnvironmentCatalogGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> TestEnvironmentCatalogGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, St>(mut self, scope: T) -> TestEnvironmentCatalogGetCall<'a, S>
                                                        where T: Into<Option<St>>,
                                                              St: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}



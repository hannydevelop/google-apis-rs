<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-privateca1_beta1` library allows access to all features of the *Google Certificate Authority Service* service.

This documentation was generated from *Certificate Authority Service* crate version *4.0.1+20220209*, where *20220209* is the exact revision of the *privateca:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v4.0.1*.

Everything else about the *Certificate Authority Service* *v1_beta1* API can be found at the
[official documentation site](https://cloud.google.com/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/CertificateAuthorityService) ... 

* projects
 * [*locations certificate authorities activate*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityActivateCall), [*locations certificate authorities certificate revocation lists get*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListGetCall), [*locations certificate authorities certificate revocation lists get iam policy*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListGetIamPolicyCall), [*locations certificate authorities certificate revocation lists list*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListListCall), [*locations certificate authorities certificate revocation lists patch*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListPatchCall), [*locations certificate authorities certificate revocation lists set iam policy*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListSetIamPolicyCall), [*locations certificate authorities certificate revocation lists test iam permissions*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevocationListTestIamPermissionCall), [*locations certificate authorities certificates create*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateCreateCall), [*locations certificate authorities certificates get*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateGetCall), [*locations certificate authorities certificates list*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateListCall), [*locations certificate authorities certificates patch*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificatePatchCall), [*locations certificate authorities certificates revoke*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCertificateRevokeCall), [*locations certificate authorities create*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityCreateCall), [*locations certificate authorities disable*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityDisableCall), [*locations certificate authorities enable*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityEnableCall), [*locations certificate authorities fetch*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityFetchCall), [*locations certificate authorities get*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityGetCall), [*locations certificate authorities get iam policy*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityGetIamPolicyCall), [*locations certificate authorities list*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityListCall), [*locations certificate authorities patch*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityPatchCall), [*locations certificate authorities restore*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityRestoreCall), [*locations certificate authorities schedule delete*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityScheduleDeleteCall), [*locations certificate authorities set iam policy*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthoritySetIamPolicyCall), [*locations certificate authorities test iam permissions*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationCertificateAuthorityTestIamPermissionCall), [*locations get*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationGetCall), [*locations list*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationOperationListCall), [*locations reusable configs get*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationReusableConfigGetCall), [*locations reusable configs get iam policy*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationReusableConfigGetIamPolicyCall), [*locations reusable configs list*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationReusableConfigListCall), [*locations reusable configs set iam policy*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationReusableConfigSetIamPolicyCall) and [*locations reusable configs test iam permissions*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/api::ProjectLocationReusableConfigTestIamPermissionCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/CertificateAuthorityService)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::CallBuilder)
* **[Resources](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_certificate_authorities_certificate_revocation_lists_patch(...).doit().await
let r = hub.projects().locations_certificate_authorities_activate(...).doit().await
let r = hub.projects().locations_certificate_authorities_create(...).doit().await
let r = hub.projects().locations_certificate_authorities_disable(...).doit().await
let r = hub.projects().locations_certificate_authorities_enable(...).doit().await
let r = hub.projects().locations_certificate_authorities_patch(...).doit().await
let r = hub.projects().locations_certificate_authorities_restore(...).doit().await
let r = hub.projects().locations_certificate_authorities_schedule_delete(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-privateca1_beta1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_privateca1_beta1 as privateca1_beta1;
use privateca1_beta1::api::CertificateRevocationList;
use privateca1_beta1::{Result, Error};
use std::default::Default;
use privateca1_beta1::{CertificateAuthorityService, oauth2, hyper, hyper_rustls};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = CertificateAuthorityService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = CertificateRevocationList::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_certificate_authorities_certificate_revocation_lists_patch(req, "name")
             .update_mask("magna")
             .request_id("no")
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::RequestValue) and 
[decodable](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-privateca1_beta1/4.0.1+20220209/google_privateca1_beta1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **privateca1_beta1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md


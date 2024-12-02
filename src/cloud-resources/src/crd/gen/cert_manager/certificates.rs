// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

#![allow(rustdoc::all)]

// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium certificates.cert-manager.io --docs --smart-derive-elision --derive Default --derive PartialEq --derive JsonSchema
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Specification of the desired state of the Certificate resource.
/// https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
#[kube(
    group = "cert-manager.io",
    version = "v1",
    kind = "Certificate",
    plural = "certificates"
)]
#[kube(namespaced)]
#[kube(status = "CertificateStatus")]
#[kube(schema = "disabled")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CertificateSpec {
    /// Defines extra output formats of the private key and signed certificate chain
    /// to be written to this Certificate's target Secret.
    ///
    /// This is a Beta Feature enabled by default. It can be disabled with the
    /// `--feature-gates=AdditionalCertificateOutputFormats=false` option set on both
    /// the controller and webhook components.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "additionalOutputFormats"
    )]
    pub additional_output_formats: Option<Vec<CertificateAdditionalOutputFormats>>,
    /// Requested common name X509 certificate subject attribute.
    /// More info: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6
    /// NOTE: TLS clients will ignore this value when any subject alternative name is
    /// set (see https://tools.ietf.org/html/rfc6125#section-6.4.4).
    ///
    /// Should have a length of 64 characters or fewer to avoid generating invalid CSRs.
    /// Cannot be set if the `literalSubject` field is set.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "commonName"
    )]
    pub common_name: Option<String>,
    /// Requested DNS subject alternative names.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsNames")]
    pub dns_names: Option<Vec<String>>,
    /// Requested 'duration' (i.e. lifetime) of the Certificate. Note that the
    /// issuer may choose to ignore the requested duration, just like any other
    /// requested attribute.
    ///
    /// If unset, this defaults to 90 days.
    /// Minimum accepted duration is 1 hour.
    /// Value must be in units accepted by Go time.ParseDuration https://golang.org/pkg/time/#ParseDuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Requested email subject alternative names.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "emailAddresses"
    )]
    pub email_addresses: Option<Vec<String>>,
    /// Whether the KeyUsage and ExtKeyUsage extensions should be set in the encoded CSR.
    ///
    /// This option defaults to true, and should only be disabled if the target
    /// issuer does not support CSRs with these X509 KeyUsage/ ExtKeyUsage extensions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encodeUsagesInRequest"
    )]
    pub encode_usages_in_request: Option<bool>,
    /// Requested IP address subject alternative names.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ipAddresses"
    )]
    pub ip_addresses: Option<Vec<String>>,
    /// Requested basic constraints isCA value.
    /// The isCA value is used to set the `isCA` field on the created CertificateRequest
    /// resources. Note that the issuer may choose to ignore the requested isCA value, just
    /// like any other requested attribute.
    ///
    /// If true, this will automatically add the `cert sign` usage to the list
    /// of requested `usages`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCA")]
    pub is_ca: Option<bool>,
    /// Reference to the issuer responsible for issuing the certificate.
    /// If the issuer is namespace-scoped, it must be in the same namespace
    /// as the Certificate. If the issuer is cluster-scoped, it can be used
    /// from any namespace.
    ///
    /// The `name` field of the reference must always be specified.
    #[serde(rename = "issuerRef")]
    pub issuer_ref: CertificateIssuerRef,
    /// Additional keystore output formats to be stored in the Certificate's Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keystores: Option<CertificateKeystores>,
    /// Requested X.509 certificate subject, represented using the LDAP "String
    /// Representation of a Distinguished Name" [1].
    /// Important: the LDAP string format also specifies the order of the attributes
    /// in the subject, this is important when issuing certs for LDAP authentication.
    /// Example: `CN=foo,DC=corp,DC=example,DC=com`
    /// More info [1]: https://datatracker.ietf.org/doc/html/rfc4514
    /// More info: https://github.com/cert-manager/cert-manager/issues/3203
    /// More info: https://github.com/cert-manager/cert-manager/issues/4424
    ///
    /// Cannot be set if the `subject` or `commonName` field is set.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "literalSubject"
    )]
    pub literal_subject: Option<String>,
    /// x.509 certificate NameConstraint extension which MUST NOT be used in a non-CA certificate.
    /// More Info: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.10
    ///
    /// This is an Alpha Feature and is only enabled with the
    /// `--feature-gates=NameConstraints=true` option set on both
    /// the controller and webhook components.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nameConstraints"
    )]
    pub name_constraints: Option<CertificateNameConstraints>,
    /// `otherNames` is an escape hatch for SAN that allows any type. We currently restrict the support to string like otherNames, cf RFC 5280 p 37
    /// Any UTF8 String valued otherName can be passed with by setting the keys oid: x.x.x.x and UTF8Value: somevalue for `otherName`.
    /// Most commonly this would be UPN set with oid: 1.3.6.1.4.1.311.20.2.3
    /// You should ensure that any OID passed is valid for the UTF8String type as we do not explicitly validate this.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "otherNames"
    )]
    pub other_names: Option<Vec<CertificateOtherNames>>,
    /// Private key options. These include the key algorithm and size, the used
    /// encoding and the rotation policy.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "privateKey"
    )]
    pub private_key: Option<CertificatePrivateKey>,
    /// How long before the currently issued certificate's expiry cert-manager should
    /// renew the certificate. For example, if a certificate is valid for 60 minutes,
    /// and `renewBefore=10m`, cert-manager will begin to attempt to renew the certificate
    /// 50 minutes after it was issued (i.e. when there are 10 minutes remaining until
    /// the certificate is no longer valid).
    ///
    /// NOTE: The actual lifetime of the issued certificate is used to determine the
    /// renewal time. If an issuer returns a certificate with a different lifetime than
    /// the one requested, cert-manager will use the lifetime of the issued certificate.
    ///
    /// If unset, this defaults to 1/3 of the issued certificate's lifetime.
    /// Minimum accepted value is 5 minutes.
    /// Value must be in units accepted by Go time.ParseDuration https://golang.org/pkg/time/#ParseDuration.
    /// Cannot be set if the `renewBeforePercentage` field is set.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renewBefore"
    )]
    pub renew_before: Option<String>,
    /// `renewBeforePercentage` is like `renewBefore`, except it is a relative percentage
    /// rather than an absolute duration. For example, if a certificate is valid for 60
    /// minutes, and  `renewBeforePercentage=25`, cert-manager will begin to attempt to
    /// renew the certificate 45 minutes after it was issued (i.e. when there are 15
    /// minutes (25%) remaining until the certificate is no longer valid).
    ///
    /// NOTE: The actual lifetime of the issued certificate is used to determine the
    /// renewal time. If an issuer returns a certificate with a different lifetime than
    /// the one requested, cert-manager will use the lifetime of the issued certificate.
    ///
    /// Value must be an integer in the range (0,100). The minimum effective
    /// `renewBefore` derived from the `renewBeforePercentage` and `duration` fields is 5
    /// minutes.
    /// Cannot be set if the `renewBefore` field is set.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renewBeforePercentage"
    )]
    pub renew_before_percentage: Option<i32>,
    /// The maximum number of CertificateRequest revisions that are maintained in
    /// the Certificate's history. Each revision represents a single `CertificateRequest`
    /// created by this Certificate, either when it was created, renewed, or Spec
    /// was changed. Revisions will be removed by oldest first if the number of
    /// revisions exceeds this number.
    ///
    /// If set, revisionHistoryLimit must be a value of `1` or greater.
    /// If unset (`nil`), revisions will not be garbage collected.
    /// Default value is `nil`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "revisionHistoryLimit"
    )]
    pub revision_history_limit: Option<i32>,
    /// Name of the Secret resource that will be automatically created and
    /// managed by this Certificate resource. It will be populated with a
    /// private key and certificate, signed by the denoted issuer. The Secret
    /// resource lives in the same namespace as the Certificate resource.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// Defines annotations and labels to be copied to the Certificate's Secret.
    /// Labels and annotations on the Secret will be changed as they appear on the
    /// SecretTemplate when added or removed. SecretTemplate annotations are added
    /// in conjunction with, and cannot overwrite, the base set of annotations
    /// cert-manager sets on the Certificate's Secret.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "secretTemplate"
    )]
    pub secret_template: Option<CertificateSecretTemplate>,
    /// Requested set of X509 certificate subject attributes.
    /// More info: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6
    ///
    /// The common name attribute is specified separately in the `commonName` field.
    /// Cannot be set if the `literalSubject` field is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<CertificateSubject>,
    /// Requested URI subject alternative names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
    /// Requested key usages and extended key usages.
    /// These usages are used to set the `usages` field on the created CertificateRequest
    /// resources. If `encodeUsagesInRequest` is unset or set to `true`, the usages
    /// will additionally be encoded in the `request` field which contains the CSR blob.
    ///
    /// If unset, defaults to `digital signature` and `key encipherment`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<String>>,
}

/// CertificateAdditionalOutputFormat defines an additional output format of a
/// Certificate resource. These contain supplementary data formats of the signed
/// certificate chain and paired private key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CertificateAdditionalOutputFormats {
    /// Type is the name of the format type that should be written to the
    /// Certificate's target Secret.
    #[serde(rename = "type")]
    pub r#type: CertificateAdditionalOutputFormatsType,
}

/// CertificateAdditionalOutputFormat defines an additional output format of a
/// Certificate resource. These contain supplementary data formats of the signed
/// certificate chain and paired private key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CertificateAdditionalOutputFormatsType {
    #[serde(rename = "DER")]
    Der,
    #[serde(rename = "CombinedPEM")]
    CombinedPem,
}

/// Reference to the issuer responsible for issuing the certificate.
/// If the issuer is namespace-scoped, it must be in the same namespace
/// as the Certificate. If the issuer is cluster-scoped, it can be used
/// from any namespace.
///
/// The `name` field of the reference must always be specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateIssuerRef {
    /// Group of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource being referred to.
    pub name: String,
}

/// Additional keystore output formats to be stored in the Certificate's Secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateKeystores {
    /// JKS configures options for storing a JKS keystore in the
    /// `spec.secretName` Secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jks: Option<CertificateKeystoresJks>,
    /// PKCS12 configures options for storing a PKCS12 keystore in the
    /// `spec.secretName` Secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkcs12: Option<CertificateKeystoresPkcs12>,
}

/// JKS configures options for storing a JKS keystore in the
/// `spec.secretName` Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateKeystoresJks {
    /// Alias specifies the alias of the key in the keystore, required by the JKS format.
    /// If not provided, the default alias `certificate` will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Create enables JKS keystore creation for the Certificate.
    /// If true, a file named `keystore.jks` will be created in the target
    /// Secret resource, encrypted using the password stored in
    /// `passwordSecretRef`.
    /// The keystore file will be updated immediately.
    /// If the issuer provided a CA certificate, a file named `truststore.jks`
    /// will also be created in the target Secret resource, encrypted using the
    /// password stored in `passwordSecretRef`
    /// containing the issuing Certificate Authority
    pub create: bool,
    /// PasswordSecretRef is a reference to a key in a Secret resource
    /// containing the password used to encrypt the JKS keystore.
    #[serde(rename = "passwordSecretRef")]
    pub password_secret_ref: CertificateKeystoresJksPasswordSecretRef,
}

/// PasswordSecretRef is a reference to a key in a Secret resource
/// containing the password used to encrypt the JKS keystore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateKeystoresJksPasswordSecretRef {
    /// The key of the entry in the Secret resource's `data` field to be used.
    /// Some instances of this field may be defaulted, in others it may be
    /// required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Name of the resource being referred to.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// PKCS12 configures options for storing a PKCS12 keystore in the
/// `spec.secretName` Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateKeystoresPkcs12 {
    /// Create enables PKCS12 keystore creation for the Certificate.
    /// If true, a file named `keystore.p12` will be created in the target
    /// Secret resource, encrypted using the password stored in
    /// `passwordSecretRef`.
    /// The keystore file will be updated immediately.
    /// If the issuer provided a CA certificate, a file named `truststore.p12` will
    /// also be created in the target Secret resource, encrypted using the
    /// password stored in `passwordSecretRef` containing the issuing Certificate
    /// Authority
    pub create: bool,
    /// PasswordSecretRef is a reference to a key in a Secret resource
    /// containing the password used to encrypt the PKCS12 keystore.
    #[serde(rename = "passwordSecretRef")]
    pub password_secret_ref: CertificateKeystoresPkcs12PasswordSecretRef,
    /// Profile specifies the key and certificate encryption algorithms and the HMAC algorithm
    /// used to create the PKCS12 keystore. Default value is `LegacyRC2` for backward compatibility.
    ///
    /// If provided, allowed values are:
    /// `LegacyRC2`: Deprecated. Not supported by default in OpenSSL 3 or Java 20.
    /// `LegacyDES`: Less secure algorithm. Use this option for maximal compatibility.
    /// `Modern2023`: Secure algorithm. Use this option in case you have to always use secure algorithms
    /// (eg. because of company policy). Please note that the security of the algorithm is not that important
    /// in reality, because the unencrypted certificate and private key are also stored in the Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<CertificateKeystoresPkcs12Profile>,
}

/// PasswordSecretRef is a reference to a key in a Secret resource
/// containing the password used to encrypt the PKCS12 keystore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateKeystoresPkcs12PasswordSecretRef {
    /// The key of the entry in the Secret resource's `data` field to be used.
    /// Some instances of this field may be defaulted, in others it may be
    /// required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Name of the resource being referred to.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// PKCS12 configures options for storing a PKCS12 keystore in the
/// `spec.secretName` Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CertificateKeystoresPkcs12Profile {
    #[serde(rename = "LegacyRC2")]
    LegacyRc2,
    #[serde(rename = "LegacyDES")]
    LegacyDes,
    Modern2023,
}

/// x.509 certificate NameConstraint extension which MUST NOT be used in a non-CA certificate.
/// More Info: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.10
///
/// This is an Alpha Feature and is only enabled with the
/// `--feature-gates=NameConstraints=true` option set on both
/// the controller and webhook components.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateNameConstraints {
    /// if true then the name constraints are marked critical.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    /// Excluded contains the constraints which must be disallowed. Any name matching a
    /// restriction in the excluded field is invalid regardless
    /// of information appearing in the permitted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded: Option<CertificateNameConstraintsExcluded>,
    /// Permitted contains the constraints in which the names must be located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted: Option<CertificateNameConstraintsPermitted>,
}

/// Excluded contains the constraints which must be disallowed. Any name matching a
/// restriction in the excluded field is invalid regardless
/// of information appearing in the permitted
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateNameConstraintsExcluded {
    /// DNSDomains is a list of DNS domains that are permitted or excluded.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dnsDomains"
    )]
    pub dns_domains: Option<Vec<String>>,
    /// EmailAddresses is a list of Email Addresses that are permitted or excluded.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "emailAddresses"
    )]
    pub email_addresses: Option<Vec<String>>,
    /// IPRanges is a list of IP Ranges that are permitted or excluded.
    /// This should be a valid CIDR notation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipRanges")]
    pub ip_ranges: Option<Vec<String>>,
    /// URIDomains is a list of URI domains that are permitted or excluded.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "uriDomains"
    )]
    pub uri_domains: Option<Vec<String>>,
}

/// Permitted contains the constraints in which the names must be located.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateNameConstraintsPermitted {
    /// DNSDomains is a list of DNS domains that are permitted or excluded.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dnsDomains"
    )]
    pub dns_domains: Option<Vec<String>>,
    /// EmailAddresses is a list of Email Addresses that are permitted or excluded.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "emailAddresses"
    )]
    pub email_addresses: Option<Vec<String>>,
    /// IPRanges is a list of IP Ranges that are permitted or excluded.
    /// This should be a valid CIDR notation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipRanges")]
    pub ip_ranges: Option<Vec<String>>,
    /// URIDomains is a list of URI domains that are permitted or excluded.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "uriDomains"
    )]
    pub uri_domains: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateOtherNames {
    /// OID is the object identifier for the otherName SAN.
    /// The object identifier must be expressed as a dotted string, for
    /// example, "1.2.840.113556.1.4.221".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    /// utf8Value is the string value of the otherName SAN.
    /// The utf8Value accepts any valid UTF8 string to set as value for the otherName SAN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "utf8Value")]
    pub utf8_value: Option<String>,
}

/// Private key options. These include the key algorithm and size, the used
/// encoding and the rotation policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificatePrivateKey {
    /// Algorithm is the private key algorithm of the corresponding private key
    /// for this certificate.
    ///
    /// If provided, allowed values are either `RSA`, `ECDSA` or `Ed25519`.
    /// If `algorithm` is specified and `size` is not provided,
    /// key size of 2048 will be used for `RSA` key algorithm and
    /// key size of 256 will be used for `ECDSA` key algorithm.
    /// key size is ignored when using the `Ed25519` key algorithm.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<CertificatePrivateKeyAlgorithm>,
    /// The private key cryptography standards (PKCS) encoding for this
    /// certificate's private key to be encoded in.
    ///
    /// If provided, allowed values are `PKCS1` and `PKCS8` standing for PKCS#1
    /// and PKCS#8, respectively.
    /// Defaults to `PKCS1` if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<CertificatePrivateKeyEncoding>,
    /// RotationPolicy controls how private keys should be regenerated when a
    /// re-issuance is being processed.
    ///
    /// If set to `Never`, a private key will only be generated if one does not
    /// already exist in the target `spec.secretName`. If one does exist but it
    /// does not have the correct algorithm or size, a warning will be raised
    /// to await user intervention.
    /// If set to `Always`, a private key matching the specified requirements
    /// will be generated whenever a re-issuance occurs.
    /// Default is `Never` for backward compatibility.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rotationPolicy"
    )]
    pub rotation_policy: Option<CertificatePrivateKeyRotationPolicy>,
    /// Size is the key bit size of the corresponding private key for this certificate.
    ///
    /// If `algorithm` is set to `RSA`, valid values are `2048`, `4096` or `8192`,
    /// and will default to `2048` if not specified.
    /// If `algorithm` is set to `ECDSA`, valid values are `256`, `384` or `521`,
    /// and will default to `256` if not specified.
    /// If `algorithm` is set to `Ed25519`, Size is ignored.
    /// No other values are allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Private key options. These include the key algorithm and size, the used
/// encoding and the rotation policy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CertificatePrivateKeyAlgorithm {
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "ECDSA")]
    Ecdsa,
    Ed25519,
}

/// Private key options. These include the key algorithm and size, the used
/// encoding and the rotation policy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CertificatePrivateKeyEncoding {
    #[serde(rename = "PKCS1")]
    Pkcs1,
    #[serde(rename = "PKCS8")]
    Pkcs8,
}

/// Private key options. These include the key algorithm and size, the used
/// encoding and the rotation policy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CertificatePrivateKeyRotationPolicy {
    Never,
    Always,
}

/// Defines annotations and labels to be copied to the Certificate's Secret.
/// Labels and annotations on the Secret will be changed as they appear on the
/// SecretTemplate when added or removed. SecretTemplate annotations are added
/// in conjunction with, and cannot overwrite, the base set of annotations
/// cert-manager sets on the Certificate's Secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateSecretTemplate {
    /// Annotations is a key value map to be copied to the target Kubernetes Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels is a key value map to be copied to the target Kubernetes Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Requested set of X509 certificate subject attributes.
/// More info: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6
///
/// The common name attribute is specified separately in the `commonName` field.
/// Cannot be set if the `literalSubject` field is set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateSubject {
    /// Countries to be used on the Certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    /// Cities to be used on the Certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localities: Option<Vec<String>>,
    /// Organizational Units to be used on the Certificate.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "organizationalUnits"
    )]
    pub organizational_units: Option<Vec<String>>,
    /// Organizations to be used on the Certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    /// Postal codes to be used on the Certificate.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "postalCodes"
    )]
    pub postal_codes: Option<Vec<String>>,
    /// State/Provinces to be used on the Certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provinces: Option<Vec<String>>,
    /// Serial number to be used on the Certificate.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "serialNumber"
    )]
    pub serial_number: Option<String>,
    /// Street addresses to be used on the Certificate.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "streetAddresses"
    )]
    pub street_addresses: Option<Vec<String>>,
}

/// Status of the Certificate.
/// This is set and managed automatically.
/// Read-only.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct CertificateStatus {
    /// List of status conditions to indicate the status of certificates.
    /// Known condition types are `Ready` and `Issuing`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The number of continuous failed issuance attempts up till now. This
    /// field gets removed (if set) on a successful issuance and gets set to
    /// 1 if unset and an issuance has failed. If an issuance has failed, the
    /// delay till the next issuance will be calculated using formula
    /// time.Hour * 2 ^ (failedIssuanceAttempts - 1).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failedIssuanceAttempts"
    )]
    pub failed_issuance_attempts: Option<i64>,
    /// LastFailureTime is set only if the latest issuance for this
    /// Certificate failed and contains the time of the failure. If an
    /// issuance has failed, the delay till the next issuance will be
    /// calculated using formula time.Hour * 2 ^ (failedIssuanceAttempts -
    /// 1). If the latest issuance has succeeded this field will be unset.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastFailureTime"
    )]
    pub last_failure_time: Option<String>,
    /// The name of the Secret resource containing the private key to be used
    /// for the next certificate iteration.
    /// The keymanager controller will automatically set this field if the
    /// `Issuing` condition is set to `True`.
    /// It will automatically unset this field when the Issuing condition is
    /// not set or False.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nextPrivateKeySecretName"
    )]
    pub next_private_key_secret_name: Option<String>,
    /// The expiration time of the certificate stored in the secret named
    /// by this resource in `spec.secretName`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notAfter")]
    pub not_after: Option<String>,
    /// The time after which the certificate stored in the secret named
    /// by this resource in `spec.secretName` is valid.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBefore")]
    pub not_before: Option<String>,
    /// RenewalTime is the time at which the certificate will be next
    /// renewed.
    /// If not set, no upcoming renewal is scheduled.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renewalTime"
    )]
    pub renewal_time: Option<String>,
    /// The current 'revision' of the certificate as issued.
    ///
    /// When a CertificateRequest resource is created, it will have the
    /// `cert-manager.io/certificate-revision` set to one greater than the
    /// current value of this field.
    ///
    /// Upon issuance, this field will be set to the value of the annotation
    /// on the CertificateRequest resource used to issue the certificate.
    ///
    /// Persisting the value on the CertificateRequest resource allows the
    /// certificates controller to know whether a request is part of an old
    /// issuance or if it is part of the ongoing revision's issuance by
    /// checking if the revision value in the annotation is greater than this
    /// field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
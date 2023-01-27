use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use log::{debug};

const DEVICE_ID_KEY: &str = "DeviceId";
const HOSTNAME_KEY: &str = "HostName";
const SHARED_ACCESS_KEY_KEY: &str = "SharedAccessKey";

// Copyright (c) Microsoft. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.

/* This file contains certs needed to communicate with Azure (IoT) */

/* Note: for devices with limited resources, only one cer    t should be loaded.
   #defines are used to reduce memory footprint of certificates.
   To use ECC with IoT Hub Gateway V2, please build with -DUSE_DIGICERT_G3_CERT.
   For DE and CN regions, please build with -DUSE_MICROSOFTAZURE_DE_CERT or -DUSE_PORTAL_AZURE_CN_CERT, respectively,
   if you wish to load ONLY those certs.
*/
// #if !defined(USE_AZURE_CLOUD_RSA_CERT) && !defined(USE_AZURE_CLOUD_ECC_CERT) && \
    // !defined(USE_MICROSOFTAZURE_DE_CERT) && !defined(USE_PORTAL_AZURE_CN_CERT)
// For legacy, if no certificates were explicitly selected then include all of them
// #define USE_AZURE_CLOUD_RSA_CERT
// #define USE_AZURE_CLOUD_ECC_CERT
// #define USE_MICROSOFTAZURE_DE_CERT
// #define USE_PORTAL_AZURE_CN_CERT
// #endif

pub const AZURE_CLOUD_RSA_CERT: &str =
/* Baltimore CyberTrust Root */
/* DigiCert Global Root G2 */
/* Microsoft RSA Root Certificate Authority 2017 */
"-----BEGIN CERTIFICATE-----\n
MIIDdzCCAl+gAwIBAgIEAgAAuTANBgkqhkiG9w0BAQUFADBaMQswCQYDVQQGEwJJ\n
RTESMBAGA2UEChMJQmFsdGltb3JlMRMwEQYDVQQLEwpDeWJlclRydXN0MSIwIAYD\n
VQQDExlCYWx1aW1vcmUgQ3liZXJUcnVzdCBSb290MB4XDTAwMDUxMjE4NDYwMFoX\n
DTI2MDUxMjIzNTkwMFowWjELMAkGA1UEBhMCSUUxEjAQBgNVBAoTCUJhbHRpbW9y\n
ZTETMBEGA2UECxMKQ3liZXJUcnVzdDEiMCAGA1UEAxMZQmFsdGltb3JlIEN5YmVy\n
VHJ2c3QgUm9vdDCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKMEuyKr\n
mD2X6CZymrV51Cni4eiVgLGw41uOKymaZN+hXe2wCQVt2yguzmKiYv60iNoS6zjr\n
IZ4AQSsBUnuId9Mcj8e6uYi1agnnc+gRQKfRzMpijS3ljwumUNKoUMMo6vWrJYeK\n
mpYcqWe5PwzV9/lSEy/CG9VwcPCPwBLKBsua4dnKM3p31vjsufFoREJIE9LAwqSu\n
XmD+tqYF/LTdB2kC1FkYmGP1pWPgkAx9XbIGevOF6uvUA65ehD5f/xXtabz5OTZy\n
dc94Uk3zyZAsuT3lySNTPx8kmCFcB5kpvcY67Oduhjprl3RjM71oGDHweI12v/ye\n
jl1qhqdNkNwnGjkCAwEAAaNFMEMwHQYDVR0OBBYEFOWdWTCCR1jMrPoIVDaGezq1\n
BE4wMBIGA1UdEwEB/wQIMAYBAf8CAQMwDgYDVR0PAQH/BAQDAgEGMA0GCSqGSIb3\n
DQEBBQUAA5IBAQCFDF2O5G9RaEIFoN27TyclhAO992T9Ldcw46QQF+vaKSm2eT92\n
10hkTI7gQCvlYpNRhcL0EYWoSihfVCr3FvDB81ukMJY2GQE/szKN+OMY3EU/t3Wgx\n
jkzSswF08r51XgdIGn9w/xZchMB5hbgF/X++ZRGjD8ACtPhSNzkE1akxehi/oCr0\n
Epn4o0WC4zxe9Z2etciefC7IpJ5OCBRLbf1wbWsaY71k5h+3zvDyny67G7fyUIhz\n
ksLi5xaNmjICq44Y3ekQEe5+NauQrz4wlHrQMz2nZQ/1/I6eYs9HRCwBXbsdtTLS\n
R9I4LtD+gdwyah617jzV/OeBHRnDJELqYzmp\n
-----END CERTIFICATE-----\n
-----BEGIN CERTIFICATE-----\n
MIIDjjCCAnagAwIBAgIQAzrx5qcRqaC7KGSxHQn65TANBgkqhkiG9w0BAQsFADBh\n
MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\n
d3cuZGlnaWNlcnQuY29tMSAwHgYDVQQDExdEaWdpQ2VydCBHbG9iYWwgUm9vdCBH\n
MjAeFw0xMzA4MDExMjAwMDBaFw0zODAxMTUxMjAwMDBaMGExCzAJBgNVBAYTAlVT\n
MRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdpY2VydC5j\n
b20xIDAeBgNVBAMTF0RpZ2lDZXJ0IEdsb2JhbCBSb290IEcyMIIBIjANBgkqhkiG\n
9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuzfNNNx7a8myaJCtSnX/RrohCgiN9RlUyfuI\n
2/Ou8jqJkTx65qsGGmvPrC3oXgkkRLpimn7Wo6h+4FR1IAWsULecYxpsMNzaHxmx\n
1x7e/dfgy5SDN67sH0NO3Xss0r0upS/kqbitOtSZpLYl6ZtrAGCSYP9PIUkY92eQ\n
q2EGnI/yuum06ZIya7XzV+hdG82MHauVBJVJ8zUtluNJbd134/tJS7SsVQepj5Wz\n
tCO7TG1F8PapspUwtP1MVYwnSlcUfIKdzXOS0xZKBgyMUNGPHgm+F6HmIcr9g+UQ\n
vIOlCsRnKPZzFBQ9RnbDhxSJITRNrw9FDKZJobq7nMWxM4MphQIDAQABo0IwQDAP\n
BgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNVHQ4EFgQUTiJUIBiV\n
5uNu5g/6+rkS7QYXjzkwDQYJKoZIhvcNAQELBQADggEBAGBnKJRvDkhj6zHd6mcY\n
1Yl9PMWLSn/pvtsrF9+wX3N3KjITOYFnQoQj8kVnNeyIv/iPsGEMNKSuIEyExtv4\n
NeF22d+mQrvHRAiGfzZ0JFrabA0UWTW98kndth/Jsw1HKj2ZL7tcu7XUIOGZX1NG\n
Fdtom/DzMNU+MeKNhJ7jitralj41E6Vf8PlwUHBHQRFXGU7Aj64GxJUTFy8bJZ91\n
8rGOmaFvE7FBcf6IKshPECBV1/MUReXgRPTqh5Uykw7+U0b6LJ3/iyK5S9kJRaTe\n
pLiaWN0bfVKfjllDiIGknibVb63dDcY3fe0Dkhvld1927jyNxF1WW6LZZm6zNTfl\n
MrY=\n
----END CERTIFICATE-----\n
-----BEGIN CERTIFICATE-----\n
MIIFqDCCA5CgAwIBAgIQHtOXCV/YtLNHcB6qvn9FszANBgkqhkiG9w0BAQwFADBl\n
MQswCQYDVQQGEwJVUzEeMBwGA1UEChMVTWljcm9zb2Z0IENvcnBvcmF0aW9uMTYw\n
NAYDVQQDEy1NaWNyb3NvZnQgUlNBIFJvb3QgQ2VydGlmaWNhdGUgQXV0aG9yaXR5\n
IDIwMTcwHhcNMTkxMjE4MjI1MTIyWhcNNDIwNzE4MjMwMDIzWjBlMQswCQYDVQQG\n
EwJVUzEeMBwGA1UEChMVTWljcm9zb2Z0IENvcnBvcmF0aW9uMTYwNAYDVQQDEy1N\n
aWNyb3NvZnQgUlNBIFJvb3QgQ2VydGlmaWNhdGUgQXV0aG9yaXR5IDIwMTcwggIi\n
MA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQDKW76UM4wplZEWCpW9R2LBifOZ\n
Nt9GkMml7Xhqb0eRaPgnZ1AzHaGm++DlQ6OEAlcBXZxIQIJTELy/xztokLaCLeX0\n
ZdDMbRnMlfl7rEqUrQ7eS0MdhweSE5CAg2Q1OQT85elss7YfUJQ4ZVBcF0a5toW1\n
HLUX6NZFndiyJrDKxHBKrmCk3bPZ7Pw71VdyvD/IybLeS2v4I2wDwAW9lcfNcztm\n
gGTjGqwu+UcF8ga2m3P1eDNbx6H7JyqhtJqRjJHTOoI+dkC0zVJhUXAoP8XFWvLJ\n
jEm7FFtNyP9nTUwSlq31/niol4fX/V4ggNyhSyL71Imtus5Hl0dVe49FyGcohJUc\n
aDDv70ngNXtk55iwlNpNhTs+VcQor1fznhPbRiefHqJeRIOkpcrVE7NLP8TjwuaG\n
YaRSMLl6IE9vDzhTyzMMEyuP1pq9KsgtsRx9S1HKR9FIJ3Jdh+vVReZIZZ2vUpC6\n
W6IYZVcSn2i51BVrlMRpIpj0M+Dt+VGOQVDJNE92kKz8OMHY4Xu54+OU4UZpyw4K\n
UGsTuqwPN1q3ErWQgR5WrlcihtnJ0tHXUeOrO8ZV/R4O03QK0dqq6mm4lyiPSMQH\n
+FJDOvTKVTUssKZqwJz58oHhEmrARdlns87/I6KJClTUFLkqqNfs+avNJVgyeY+Q\n
W5g5xAgGwax/Dj0ApQIDAQABo1QwUjAOBgNVHQ8BAf8EBAMCAYYwDwYDVR0TAQH/\n
BAUwAwEB/zAdBgNVHQ4EFgQUCctZf4aycI8awznjwNnpv7tNsiMwEAYJKwYBBAGC\n
NxUBBAMCAQAwDQYJKoZIhvcNAQEMBQADggIBAKyvPl3CEZaJjqPnktaXFbgToqZC\n
LgLNFgVZJ8og6Lq46BrsTaiXVq5lQ7GPAJtSzVXNUzltYkyLDVt8LkS/gxCP81OC\n
gMNPOsduET/m4xaRhPtthH80dK2Jp86519efhGSSvpWhrQlTM93uCupKUY5vVau6\n
tZRGrox/2KJQJWVggEbbMwSubLWYdFQl3JPk+ONVFT24bcMKpBLBaYVu32TxU5nh\n
SnUgnZUP5NbcA/FZGOhHibJXWpS2qdgXKxdJ5XbLwVaZOjex/2kskZGT4d9Mozd2\n
TaGf+G0eHdP67Pv0RR0Tbc/3WeUiJ3IrhvNXuzDtJE3cfVa7o7P4NHmJweDyAmH3\n
pvwPuxwXC65B2Xy9J6P9LjrRk5Sxcx0ki69bIImtt2dmefU6xqaWM/5TkshGsRGR\n
xpl/j8nWZjEgQRCHLQzWwa80mMpkg/sTV9HB8Dx6jKXB/ZUhoHHBk2dxEuqPiApp\n
GWSZI1b7rCoucL5mxAyE7+WL85MB+GqQk2dLsmijtWKP6T+MejteD+eMuMZ87zf9\n
dOLITzNy4ZQ5bb0Sr74MTnB8G2+NszKTc0QWbej09+CVgI+WXTik9KveCjCHk9hN\n
AHFiRSdLOkKEW39lt2c0Ui2cFmuqqNh7o0JMcccMyj6D5KbvtwEwXlGjefVwaaZB\n
RA+GsCyRxj3qrg+E\n
-----END CERTIFICATE-----";

// #if defined(USE_AZURE_CLOUD_ECC_CERT)
// DigiCert Global Root G3
// Microsoft ECC Root Certificate Authority 2017
pub const AZURE_CLOUD_ECC_CERT: &str =
"-----BEGIN CERTIFICATE-----\n
MIICPzCCAcWgAwIBAgIQBVVWvPJepDU1w6QP1atFcjAKBggqhkjOPQQDAzBhMQsw\n
CQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3d3cu\n
ZGlnaWNlcnQuY29tMSAwHgYDVQQDExdEaWdpQ2VydCBHbG9iYWwgUm9vdCBHMzAe\n
Fw0xMzA4MDExMjAwMDBaFw0zODAxMTUxMjAwMDBaMGExCzAJBgNVBAYTAlVTMRUw\n
EwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdpY2VydC5jb20x\n
IDAeBgNVBAMTF0RpZ2lDZXJ0IEdsb2JhbCBSb290IEczMHYwEAYHKoZIzj0CAQYF\n
K4EEACIDYgAE3afZu4q4C/sLfyHS8L6+c/MzXRq8NOrexpu80JX28MzQC7phW1FG\n
fp4tn+6OYwwX7Adw9c+ELkCDnOg/QW07rdOkFFk2eJ0DQ+4QE2xy3q6Ip6FrtUPO\n
Z9wj/wMco+I+o0IwQDAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAd\n
BgNVHQ4EFgQUs9tIpPmhxdiuNkHMEWNpYim8S8YwCgYIKoZIzj0EAwMDaAAwZQIx\n
AK288mw/EkrRLTnDCgmXc/SINoyIJ7vmiI1Qhadj+Z4y3maTD/HMsQmP3Wyr+mt/\n
oAIwOWZbwmSNuJ5Q3KjVSaLtx9zRSX8XAbjIho9OjIgrqJqpisXRAL34VOKa5Vt8\n
sycX\n
-----END CERTIFICATE-----\n
-----BEGIN CERTIFICATE-----\n
MIICWTCCAd+gAwIBAgIQZvI9r4fei7FK6gxXMQHC7DAKBggqhkjOPQQDAzBlMQsw\n
CQYDVQQGEwJVUzEeMBwGA1UEChMVTWljcm9zb2Z0IENvcnBvcmF0aW9uMTYwNAYD\n
VQQDEy1NaWNyb3NvZnQgRUNDIFJvb3QgQ2VydGlmaWNhdGUgQXV0aG9yaXR5IDIw\n
MTcwHhcNMTkxMjE4MjMwNjQ1WhcNNDIwNzE4MjMxNjA0WjBlMQswCQYDVQQGEwJV\n
UzEeMBwGA1UEChMVTWljcm9zb2Z0IENvcnBvcmF0aW9uMTYwNAYDVQQDEy1NaWNy\n
b3NvZnQgRUNDIFJvb3QgQ2VydGlmaWNhdGUgQXV0aG9yaXR5IDIwMTcwdjAQBgcq\n
hkjOPQIBBgUrgQQAIgNiAATUvD0CQnVBEyPNgASGAlEvaqiBYgtlzPbKnR5vSmZR\n
ogPZnZH6thaxjG7efM3beaYvzrvOcS/lpaso7GMEZpn4+vKTEAXhgShC48Zo9OYb\n
hGBKia/teQ87zvH2RPUBeMCjVDBSMA4GA1UdDwEB/wQEAwIBhjAPBgNVHRMBAf8E\n
BTADAQH/MB0GA1UdDgQWBBTIy5lycFIM+Oa+sgRXKSrPQhDtNTAQBgkrBgEEAYI3\n
FQEEAwIBADAKBggqhkjOPQQDAwNoADBlAjBY8k3qDPlfXu5gKcs68tvWMoQZP3zV\n
L8KxzJOuULsJMsbG7X7JNpQS5GiFBqIb0C8CMQCZ6Ra0DvpWSNSkMBaReNtUjGUB\n
iudQZsIxtzm6uBoiB078a1QWIP8rtedMDE2mT3M=\n
-----END CERTIFICATE-----\n";
//#endif /* USE_AZURE_CLOUD_ECC_CERT */

// #if defined(USE_MICROSOFTAZURE_DE_CERT)
// /* D-TRUST Root Class 3 CA 2 2009 */
// // This cert should be used when connecting to Azure IoT on the https://portal.microsoftazure.de Cloud address.
pub const MICROSOFTAZURE_DE_CERT: &str =
"-----BEGIN CERTIFICATE-----\n
MIIEMzCCAxugAwIBAgIDCYPzMA0GCSqGSIb3DQEBCwUAME0xCzAJBgNVBAYTAkRF\n
MRUwEwYDVQQKDAxELVRydXN0IEdtYkgxJzAlBgNVBAMMHkQtVFJVU1QgUm9vdCBD\n
bGFzcyAzIENBIDIgMjAwOTAeFw0wOTExMDUwODM1NThaFw0yOTExMDUwODM1NTha\n
ME0xCzAJBgNVBAYTAkRFMRUwEwYDVQQKDAxELVRydXN0IEdtYkgxJzAlBgNVBAMM\n
HkQtVFJVU1QgUm9vdCBDbGFzcyAzIENBIDIgMjAwOTCCASIwDQYJKoZIhvcNAQEB\n
BQADggEPADCCAQoCggEBANOySs96R+91myP6Oi/WUEWJNTrGa9v+2wBoqOADER03\n
UAifTUpolDWzU9GUY6cgVq/eUXjsKj3zSEhQPgrfRlWLJ23DEE0NkVJD2IfgXU42\n
tSHKXzlABF9bfsyjxiupQB7ZNoTWSPOSHjRGICTBpFGOShrvUD9pXRl/RcPHAY9R\n
ySPocq60vFYJfxLLHLGvKZAKyVXMD9O0Gu1HNVpK7ZxzBCHQqr0ME7UAyiZsxGsM\n
lFqVlNpQmvH/pStmMaTJOKDfHR+4CS7zp+hnUquVH+BGPtikw8paxTGA6Eian5Rp\n
/hnd2HN8gcqW3o7tszIFZYQ05ub9VxC1X3a/L7AQDcUCAwEAAaOCARowggEWMA8G\n
A1UdEwEB/wQFMAMBAf8wHQYDVR0OBBYEFP3aFMSfMN4hvR5COfyrYyNJ4PGEMA4G\n
A1UdDwEB/wQEAwIBBjCB0wYDVR0fBIHLMIHIMIGAoH6gfIZ6bGRhcDovL2RpcmVj\n
dG9yeS5kLXRydXN0Lm5ldC9DTj1ELVRSVVNUJTIwUm9vdCUyMENsYXNzJTIwMyUy\n
MENBJTIwMiUyMDIwMDksTz1ELVRydXN0JTIwR21iSCxDPURFP2NlcnRpZmljYXRl\n
cmV2b2NhdGlvbmxpc3QwQ6BBoD+GPWh0dHA6Ly93d3cuZC10cnVzdC5uZXQvY3Js\n
L2QtdHJ1c3Rfcm9vdF9jbGFzc18zX2NhXzJfMjAwOS5jcmwwDQYJKoZIhvcNAQEL\n
BQADggEBAH+X2zDI36ScfSF6gHDOFBJpiBSVYEQBrLLpME+bUMJm2H6NMLVwMeni\n
acfzcNsgFYbQDfC+rAF1hM5+n02/t2A7nPPKHeJeaNijnZflQGDSNiH+0LS4F9p0\n
o3/U37CYAqxva2ssJSRyoWXuJVrl5jLn8t+rSfrzkGkj2wTZ51xY/GXUl77M/C4K\n
zCUqNQT4YJEVdT1B/yMfGchs64JTBKbkTCJNjYy6zltz7GRUUG3RnFX7acM2w4y8\n
PIWmawomDeCTmGCufsYkl4phX5GOZpIJhzbNi5stPvZR1FDUWSi9g/LMKHtThm3Y\n
Johw1+qRzT65ysCQblrGXnRl11z+o+I=\n
-----END CERTIFICATE-----\n";
// #endif /* MICROSOFTAZURE_DE_CERT */

// #if defined(USE_PORTAL_AZURE_CN_CERT)
/* DigiCert Global Root CA */
// This cert should be used when connecting to Azure IoT on the https://portal.azure.cn Cloud address.
pub const PORTAL_AZURE_CN_CERT: &str =
"-----BEGIN CERTIFICATE-----\n
MIIDrzCCApegAwIBAgIQCDvgVpBCRrGhdWrJWZHHSjANBgkqhkiG9w0BAQUFADBh\n
MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\n
d3cuZGlnaWNlcnQuY29tMSAwHgYDVQQDExdEaWdpQ2VydCBHbG9iYWwgUm9vdCBD\n
QTAeFw0wNjExMTAwMDAwMDBaFw0zMTExMTAwMDAwMDBaMGExCzAJBgNVBAYTAlVT\n
MRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdpY2VydC5j\n
b20xIDAeBgNVBAMTF0RpZ2lDZXJ0IEdsb2JhbCBSb290IENBMIIBIjANBgkqhkiG\n
9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4jvhEXLeqKTTo1eqUKKPC3eQyaKl7hLOllsB\n
CSDMAZOnTjC3U/dDxGkAV53ijSLdhwZAAIEJzs4bg7/fzTtxRuLWZscFs3YnFo97\n
nh6Vfe63SKMI2tavegw5BmV/Sl0fvBf4q77uKNd0f3p4mVmFaG5cIzJLv07A6Fpt\n
43C/dxC//AH2hdmoRBBYMql1GNXRor5H4idq9Joz+EkIYIvUX7Q6hL+hqkpMfT7P\n
T19sdl6gSzeRntwi5m3OFBqOasv+zbMUZBfHWymeMr/y7vrTC0LUq7dBMtoM1O/4\n
gdW7jVg/tRvoSSiicNoxBN33shbyTApOB6jtSj1etX+jkMOvJwIDAQABo2MwYTAO\n
BgNVHQ8BAf8EBAMCAYYwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUA95QNVbR\n
TLtm8KPiGxvDl7I90VUwHwYDVR0jBBgwFoAUA95QNVbRTLtm8KPiGxvDl7I90VUw\n
DQYJKoZIhvcNAQEFBQADggEBAMucN6pIExIK+t1EnE9SsPTfrgT1eXkIoyQY/Esr\n
hMAtudXH/vTBH1jLuG2cenTnmCmrEbXjcKChzUyImZOMkXDiqw8cvpOp/2PV5Adg\n
06O/nVsJ8dWO41P0jmP6P6fbtGbfYmbW0W5BjfIttep3Sp+dWOIrWcBAI+0tKIJF\n
PnlUkiaY4IBIqDfv8NZ5YBberOgOzW6sRBc4L0na4UU+Krk2U886UAb3LujEV0ls\n
YSEY1QSteDwsOoBrp+uvFRTp2InBuThs4pFsiv9kuXclVzDAGySj4dzp30d8tbQk\n
CAUw7C29C79Fv1C5qfPrmAESrciIxpg0X40KPMbp1ZWVbd4=\n
-----END CERTIFICATE-----\n";
// #endif /* PORTAL_AZURE_CN_CERT */


#[derive(Debug)]
pub enum TokenError {
    ///
    ConnectionStringMissingRequiredParameter(&'static str),
    ///
    InvalidKeyFormat,
    ///
    InvalidKeyLength,
}

pub struct Auth {
    connection_string: String,
    device_id: String,
    hostname: String,
    shared_access_key: String,
}

impl Auth {
    pub fn new(connection_string: &str) -> Result<Auth, TokenError> {
        let connection_string_parts = Auth::get_parts_of_connection_string(&connection_string)?;

        Auth::validate_key(connection_string_parts.2)?;

        Ok(Auth {
            connection_string: String::from(connection_string),
            device_id: String::from(connection_string_parts.0),
            hostname: String::from(connection_string_parts.1),
            shared_access_key: String::from(connection_string_parts.2),
        })
    }

    pub fn clone(&self) -> Auth {
        Auth {
            connection_string: self.connection_string.clone(),
            device_id: self.device_id.clone(),
            hostname: self.hostname.clone(),
            shared_access_key: self.shared_access_key.clone()
        }
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn generate_sas_token(&self, expiry: &DateTime<Utc>) -> anyhow::Result<String> {
        let expiry_timestamp = expiry.timestamp();

        let resource_uri = format!("{}%2Fdevices%2F{}", self.hostname, self.device_id);

        let to_sign = format!("{}\n{}", &resource_uri, expiry_timestamp);

        let token = Auth::generate_token(&self.shared_access_key, &to_sign)?;

        let sas = format!(
            "SharedAccessSignature sr={}&{}&se={}",
            resource_uri, token, expiry_timestamp
        );

        debug!("SAS token: {}", sas);

        return Ok(sas);
    }

    fn generate_token(key: &str, message: &str) -> anyhow::Result<String> {
        // Checked base64 and hmac in new so should be safe to unwrap here
        let key = base64::decode(&key)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(&key)?;
        mac.update(message.as_bytes());
        let mac_result = mac.finalize();
        let signature = base64::encode(mac_result.into_bytes());

        let pairs = &vec![("sig", signature)];
        let ret = serde_urlencoded::to_string(pairs)?;
        return Ok(ret);
    }

    fn get_parts_of_connection_string(
        connection_string: &str,
    ) -> Result<(&str, &str, &str), TokenError> {
        let mut device_id = None;
        let mut hostname = None;
        let mut shared_access_key = None;

        let connection_string_parts: Vec<&str> = connection_string.split(";").collect();

        for parts in connection_string_parts {
            debug!("Key value: {}", parts);

            let key_value_part: Vec<&str> = parts.splitn(2, "=").collect();

            if key_value_part.len() == 2 {
                debug!(
                    "Key value pair: key:{} value:{}",
                    key_value_part[0], key_value_part[1]
                );

                match key_value_part[0] {
                    DEVICE_ID_KEY => device_id = Some(key_value_part[1]),
                    HOSTNAME_KEY => hostname = Some(key_value_part[1]),
                    SHARED_ACCESS_KEY_KEY => shared_access_key = Some(key_value_part[1]),
                    _ => (),
                }
            }
        }

        let device_id = device_id.ok_or(TokenError::ConnectionStringMissingRequiredParameter(
            DEVICE_ID_KEY,
        ))?;

        let hostname = hostname.ok_or(TokenError::ConnectionStringMissingRequiredParameter(
            HOSTNAME_KEY,
        ))?;

        let shared_access_key = shared_access_key.ok_or(
            TokenError::ConnectionStringMissingRequiredParameter(SHARED_ACCESS_KEY_KEY),
        )?;

        Ok((device_id, hostname, shared_access_key))
    }

    fn validate_key(key: &str) -> Result<bool, TokenError> {
        // Verify key is base64
        let b64_key = base64::decode(&key).map_err(|_| TokenError::InvalidKeyFormat)?;
        // Verify key is the right length for Hmac
        Hmac::<Sha256>::new_from_slice(&b64_key).map_err(|_| TokenError::InvalidKeyLength)?;

        Ok(true)
    }
}

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;
//     use chrono::prelude::*;

//     #[test]
//     fn test_sas_source() {
//         // Should pass value through unchanged
//         let sas_token_source = SasTokenSource::new("MySasString");
//         assert_eq!(sas_token_source.get(&Utc::now()), "MySasString".to_string());
//     }

//     #[test]
//     fn test_key_source() {
//         let key_source = DeviceKeyTokenSource::new(
//             "pontifex.azure-devices.net",
//             "FirstDevice",
//             "O+H9VTcdJP0TQkl7bh4nVG0OJNrEataMpuWB54D0VEc=",
//         )
//         .unwrap();
//         let expiry = Utc.ymd(2020, 6, 28).and_hms(14, 08, 25);
//         assert_eq!(key_source.get(&expiry), "SharedAccessSignature sr=pontifex.azure-devices.net%2Fdevices%2FFirstDevice&sig=CKYVArtLm72J2UNWLb4V3XqPc679Ig3LX83G3nPExUc%3D&se=1593353305");
//     }
// }

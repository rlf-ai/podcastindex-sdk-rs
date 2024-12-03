# \AddApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_slash_byfeedurl_get**](AddApi.md#add_slash_byfeedurl_get) | **GET** /add/byfeedurl | By Feed URL
[**add_slash_byfeedurl_post**](AddApi.md#add_slash_byfeedurl_post) | **POST** /add/byfeedurl | By Feed URL
[**add_slash_byitunesid_get**](AddApi.md#add_slash_byitunesid_get) | **GET** /add/byitunesid | By iTunes ID
[**add_slash_byitunesid_post**](AddApi.md#add_slash_byitunesid_post) | **POST** /add/byitunesid | By iTunes ID



## add_slash_byfeedurl_get

> models::AddByfeedurlGet200Response add_slash_byfeedurl_get(url, chash, itunesid, pretty)
By Feed URL

This call adds a podcast to the index using its feed url. If a feed already exists, you will get its existing Feed ID returned.   **NOTE**: this endpoint requires an API Key with the **write** permission.   Example: https://api.podcastindex.org/api/1.0/add/byfeedurl?url=https://feeds.theincomparable.com/batmanuniversity&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Podcast feed URL  | [required] |
**chash** | **String** | The md5 hash of the following feed items in hex format. If known, allows for easier duplicate checking.    - `title`   - `link`   - `feedLanguage`   - `generator`   - `author`   - `ownerName`   - `ownerEmail` (note: not exposed via the API)  Pseudo-code:        chash = md5(title+link+feedLanguage+generator+author+ownerName+ownerEmail)  | [required] |
**itunesid** | Option<**i32**> | If this parameter is given, and the existing feed has no associated iTunes ID, it will be associated with this ID. If an existing iTunes ID is already associated with this feed it will **NOT** be changed.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::AddByfeedurlGet200Response**](add_byfeedurl_get_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_slash_byfeedurl_post

> models::AddByfeedurlGet200Response add_slash_byfeedurl_post(url, itunesid, pretty)
By Feed URL

This call adds a podcast to the index using its feed url. If a feed already exists, you will get its existing Feed ID returned.   **NOTE**: this endpoint requires an API Key with the **write** permission.   Example: https://api.podcastindex.org/api/1.0/add/byfeedurl?url=https://feeds.theincomparable.com/batmanuniversity&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Podcast feed URL  | [required] |
**itunesid** | Option<**i32**> | If this parameter is given, and the existing feed has no associated iTunes ID, it will be associated with this ID. If an existing iTunes ID is already associated with this feed it will **NOT** be changed.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::AddByfeedurlGet200Response**](add_byfeedurl_get_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_slash_byitunesid_get

> models::Search400Response add_slash_byitunesid_get(id, pretty)
By iTunes ID

This call adds a podcast to the index using its iTunes ID. If a feed already exists, it will be noted in the response.   **NOTE**: this endpoint requires an API Key with the **write** permission.   Example: https://api.podcastindex.org/api/1.0/add/byitunesid?id=1441923632&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i32**> | The iTunes ID to add  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::Search400Response**](search_400_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_slash_byitunesid_post

> models::Search400Response add_slash_byitunesid_post(id, pretty)
By iTunes ID

This call adds a podcast to the index using its iTunes ID. If a feed already exists, it will be noted in the response.   **NOTE**: this endpoint requires an API Key with the **write** permission.   Example: https://api.podcastindex.org/api/1.0/add/byitunesid?id=1441923632&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i32**> | The iTunes ID to add  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::Search400Response**](search_400_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AppleReplacementApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lookup**](AppleReplacementApi.md#lookup) | **GET** /lookup | Lookup
[**search**](AppleReplacementApi.md#search) | **GET** /search | Search



## lookup

> models::Search200Response lookup(id, entity, pretty)
Lookup

Replaces the Apple podcast lookup API but returns data from the Podcast Index database.   Note: No API key needed for this endpoint.   Example:    - Apple: https://itunes.apple.com/lookup?media=podcast&entity=podcast&id=1636765656   - PodcastIndex: https://api.podcastindex.org/lookup?entity=podcast&id=1636765656 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The iTunes Feed ID to search for  | [required] |
**entity** | **String** | The iTunes entity type to look in  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::Search200Response**](search_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> models::Search200Response search(term, pretty)
Search

Replaces the Apple search API but returns data from the Podcast Index database.   Note: No API key needed for this endpoint.   Example:    - Apple: https://itunes.apple.com/search?media=podcast&entity=podcast&term=batman   - PodcastIndex: https://api.podcastindex.org/search?term=batman 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The term to search for  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::Search200Response**](search_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \SearchApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_slash_byperson**](SearchApi.md#search_slash_byperson) | **GET** /search/byperson | Search Episodes by Person
[**search_slash_byterm**](SearchApi.md#search_slash_byterm) | **GET** /search/byterm | Search Podcasts
[**search_slash_bytitle**](SearchApi.md#search_slash_bytitle) | **GET** /search/bytitle | Search Podcasts by Title
[**search_slash_music_slash_byterm**](SearchApi.md#search_slash_music_slash_byterm) | **GET** /search/music/byterm | Search Music Podcasts



## search_slash_byperson

> models::SearchByperson200Response search_slash_byperson(q, max, fulltext, pretty)
Search Episodes by Person

This call returns all of the episodes where the specified person is mentioned.   It searches the following fields:    - Person tags   - Episode title   - Episode description   - Feed owner   - Feed author   Examples:    - https://api.podcastindex.org/api/1.0/search/byperson?q=adam%20curry&pretty   - https://api.podcastindex.org/api/1.0/search/byperson?q=Martin+Mouritzen&pretty   - https://api.podcastindex.org/api/1.0/search/byperson?q=Klaus+Schwab&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Person search for  | [required] |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::SearchByperson200Response**](search_byperson_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_slash_byterm

> models::SearchByterm200Response search_slash_byterm(q, val, max, aponly, clean, similar, fulltext, pretty)
Search Podcasts

This call returns all of the feeds that match the search terms in the `title`, `author` or `owner` of the feed.   Example: https://api.podcastindex.org/api/1.0/search/byterm?q=batman+university&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Terms to search for  | [required] |
**val** | Option<**String**> | Only returns feeds with a `value` block of the specified type. Use `any` to return feeds with any `value` block.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**aponly** | Option<**bool**> | Only returns feeds with an `itunesId`.  |  |
**clean** | Option<**bool**> | If present, only non-explicit feeds will be returned. Meaning, feeds where the `itunes:explicit` flag is set to `false`.   Parameter shall not have a value  |  |
**similar** | Option<**bool**> | If present, include similar matches in search response. For `search/byterm`, prioritizes title matches.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::SearchByterm200Response**](search_byterm_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_slash_bytitle

> models::SearchByterm200Response search_slash_bytitle(q, val, max, clean, fulltext, pretty, similar)
Search Podcasts by Title

This call returns all of the feeds where the `title` of the feed matches the search term (ignores case).   Example \"everything everywhere daily\" will match the podcast [Everything Everywhere Daily](https://podcastindex.org/podcast/437685) by \"everything everywhere\" will not.   Example: https://api.podcastindex.org/api/1.0/search/bytitle?q=everything+everywhere+daily&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Terms to search for  | [required] |
**val** | Option<**String**> | Only returns feeds with a `value` block of the specified type. Use `any` to return feeds with any `value` block.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**clean** | Option<**bool**> | If present, only non-explicit feeds will be returned. Meaning, feeds where the `itunes:explicit` flag is set to `false`.   Parameter shall not have a value  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |
**similar** | Option<**bool**> | If present, include similar matches in search response. For `search/byterm`, prioritizes title matches.  |  |

### Return type

[**models::SearchByterm200Response**](search_byterm_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_slash_music_slash_byterm

> models::SearchByterm200Response search_slash_music_slash_byterm(q, val, aponly, max, clean, fulltext, pretty)
Search Music Podcasts

This call returns all of the feeds that match the search terms in the `title`, `author` or `owner` of the <feed></feed> where the [medium](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#medium) is `music`.   Example: https://api.podcastindex.org/api/1.0/search/music/byterm?q=able+kirby&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Terms to search for  | [required] |
**val** | Option<**String**> | Only returns feeds with a `value` block of the specified type. Use `any` to return feeds with any `value` block.  |  |
**aponly** | Option<**bool**> | Only returns feeds with an `itunesId`.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**clean** | Option<**bool**> | If present, only non-explicit feeds will be returned. Meaning, feeds where the `itunes:explicit` flag is set to `false`.   Parameter shall not have a value  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::SearchByterm200Response**](search_byterm_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


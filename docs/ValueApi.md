# \ValueApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**value_slash_batch_slash_byepisodeguid**](ValueApi.md#value_slash_batch_slash_byepisodeguid) | **POST** /value/batch/byepisodeguid | Batch By Episode GUID
[**value_slash_byepisodeguid**](ValueApi.md#value_slash_byepisodeguid) | **GET** /value/byepisodeguid | By Episode GUID
[**value_slash_byfeedid**](ValueApi.md#value_slash_byfeedid) | **GET** /value/byfeedid | By Feed ID
[**value_slash_byfeedurl**](ValueApi.md#value_slash_byfeedurl) | **GET** /value/byfeedurl | By Feed URL
[**value_slash_bypodcastguid**](ValueApi.md#value_slash_bypodcastguid) | **GET** /value/bypodcastguid | By Feed GUID



## value_slash_batch_slash_byepisodeguid

> models::ValueBatchByepisodeguid200Response value_slash_batch_slash_byepisodeguid(request_body, pretty)
Batch By Episode GUID

This call returns the information for supporting the podcast episode via one of the \"Value for Value\" methods from a JSON object containing one or more podcast GUID and one or more episode GUID for the podcast.   The JSON object key shall be the `podcastguid` from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast. See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.   The value of the `podcastguid` shall be an array of `episodeguid` values. This is the unique guid specified for the `<item>` in the feed but may not be globally unique.   Note: No API key needed for this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**std::collections::HashMap<String, Vec<String>>**](Vec.md) | Get episode value data | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::ValueBatchByepisodeguid200Response**](value_batch_byepisodeguid_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## value_slash_byepisodeguid

> models::ValueByepisodeguid200Response value_slash_byepisodeguid(podcastguid, episodeguid, pretty)
By Episode GUID

This call returns the information for supporting the podcast episode via one of the \"Value for Value\" methods from podcast GUID and the episode GUID.   The `podcastguid` is the GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast. See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.   The `episodeguid` is the unique guid specified for the `<item>` in the feed but may not be globally unique.   Note: No API key needed for this endpoint.   Examples:    - https://api.podcastindex.org/api/1.0/value/byepisodeguid?podcastguid=917393e3-1b1e-5cef-ace4-edaa54e1f810&episodeguid=PC20143&pretty   - https://api.podcastindex.org/api/1.0/value/byepisodeguid?podcastguid=c73b1a23-1c28-5edb-94c3-10d1745d0877&episodeguid=bdea6759-a7b6-4c0d-9d1e-acca3133f4a9&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**podcastguid** | **String** | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [required] |
**episodeguid** | **String** | The guid specified by the `<guid>` in the episode `<item>`.  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::ValueByepisodeguid200Response**](value_byepisodeguid_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## value_slash_byfeedid

> models::ValueByfeedid200Response value_slash_byfeedid(id, pretty)
By Feed ID

This call returns the information for supporting the podcast via one of the \"Value for Value\" methods from the PodcastIndex ID.   Additionally, the value block data can be accessed using static JSON files (updated every 15 minutes).    - Feeds: https://tracking.podcastindex.org/feedValueBlocks.json   - Episodes: https://tracking.podcastindex.org/episodeValueBlocks.json   Note: No API key needed for this endpoint.   Examples:    - https://api.podcastindex.org/api/1.0/value/byfeedid?id=920666&pretty   - https://api.podcastindex.org/api/1.0/value/byfeedid?id=779873&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The PodcastIndex Feed ID to search for.  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::ValueByfeedid200Response**](value_byfeedid_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## value_slash_byfeedurl

> models::ValueByfeedurl200Response value_slash_byfeedurl(url, pretty)
By Feed URL

This call returns the information for supporting the podcast via one of the \"Value for Value\" methods from feed URL.   Additionally, the value block data can be accessed using static JSON files (updated every 15 minutes).    - Feeds: https://tracking.podcastindex.org/feedValueBlocks.json   - Episodes: https://tracking.podcastindex.org/episodeValueBlocks.json   Note: No API key needed for this endpoint.   Examples:    - https://api.podcastindex.org/api/1.0/value/byfeedurl?url=https://mp3s.nashownotes.com/pc20rss.xml&pretty   - https://api.podcastindex.org/api/1.0/value/byfeedurl?url=https://lespoesiesdheloise.fr/@heloise/feed.xml&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Podcast feed URL  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::ValueByfeedurl200Response**](value_byfeedurl_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## value_slash_bypodcastguid

> models::ValueBypodcastguid200Response value_slash_bypodcastguid(guid, pretty)
By Feed GUID

This call returns the information for supporting the podcast via one of the \"Value for Value\" methods from podcast GUID.   Note: No API key needed for this endpoint.   Example: https://api.podcastindex.org/api/1.0/value/bypodcastguid?guid=917393e3-1b1e-5cef-ace4-edaa54e1f810&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guid** | **String** | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::ValueBypodcastguid200Response**](value_bypodcastguid_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


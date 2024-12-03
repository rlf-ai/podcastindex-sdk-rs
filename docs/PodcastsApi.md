# \PodcastsApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**podcasts_slash_byfeedid**](PodcastsApi.md#podcasts_slash_byfeedid) | **GET** /podcasts/byfeedid | By Feed ID
[**podcasts_slash_byfeedurl**](PodcastsApi.md#podcasts_slash_byfeedurl) | **GET** /podcasts/byfeedurl | By Feed URL
[**podcasts_slash_byguid**](PodcastsApi.md#podcasts_slash_byguid) | **GET** /podcasts/byguid | By GUID
[**podcasts_slash_byitunesid**](PodcastsApi.md#podcasts_slash_byitunesid) | **GET** /podcasts/byitunesid | By iTunes ID
[**podcasts_slash_bymedium**](PodcastsApi.md#podcasts_slash_bymedium) | **GET** /podcasts/bymedium | By Medium
[**podcasts_slash_bytag**](PodcastsApi.md#podcasts_slash_bytag) | **GET** /podcasts/bytag | By Tag
[**podcasts_slash_dead**](PodcastsApi.md#podcasts_slash_dead) | **GET** /podcasts/dead | Dead
[**podcasts_slash_trending**](PodcastsApi.md#podcasts_slash_trending) | **GET** /podcasts/trending | Trending



## podcasts_slash_byfeedid

> models::PodcastsByfeedid200Response podcasts_slash_byfeedid(id, pretty)
By Feed ID

This call returns everything we know about the feed from the PodcastIndex ID   Examples:    - https://api.podcastindex.org/api/1.0/podcasts/byfeedid?id=75075&pretty   - Includes `value` and `funding`: https://api.podcastindex.org/api/1.0/podcasts/byfeedid?id=169991&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The PodcastIndex Feed ID  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsByfeedid200Response**](podcasts_byfeedid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_byfeedurl

> models::PodcastsByfeedurl200Response podcasts_slash_byfeedurl(url, pretty)
By Feed URL

This call returns everything we know about the feed from the feed URL   Examples:    - https://api.podcastindex.org/api/1.0/podcasts/byfeedurl?url=https://feeds.theincomparable.com/batmanuniversity&pretty   - Includes `value` and `funding`: https://api.podcastindex.org/api/1.0/podcasts/byfeedurl?url=https://engineered.network/pragmatic/feed/index.xml&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Podcast feed URL  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsByfeedurl200Response**](podcasts_byfeedurl_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_byguid

> models::PodcastsByguid200Response podcasts_slash_byguid(guid, pretty)
By GUID

This call returns everything we know about the feed from the feed's GUID.   The GUID is a unique, global identifier for the podcast. See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.   Examples:    - https://api.podcastindex.org/api/1.0/podcasts/byguid?guid=9b024349-ccf0-5f69-a609-6b82873eab3c&pretty   - Includes `value` and `funding`: https://api.podcastindex.org/api/1.0/podcasts/byguid?guid=9b024349-ccf0-5f69-a609-6b82873eab3c&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guid** | **String** | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsByguid200Response**](podcasts_byguid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_byitunesid

> models::PodcastsByitunesid200Response podcasts_slash_byitunesid(id, pretty)
By iTunes ID

This call returns everything we know about the feed from the iTunes ID   Example: https://api.podcastindex.org/api/1.0/podcasts/byitunesid?id=1441923632&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The iTunes Feed ID to search for  | [required] |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsByitunesid200Response**](podcasts_byitunesid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_bymedium

> models::PodcastsBymedium200Response podcasts_slash_bymedium(medium, max, pretty)
By Medium

This call returns all feeds marked with the specified [medium](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#medium) tag value.   Example: https://api.podcastindex.org/api/1.0/podcasts/bymedium?medium=music&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**medium** | Option<**String**> | The medium value to search for.   Full list of possible values documented in [medium](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#medium) tag spec.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsBymedium200Response**](podcasts_bymedium_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_bytag

> models::PodcastsBytag200Response podcasts_slash_bytag(podcast_value, podcast_value_time_split, max, start_at, pretty)
By Tag

This call returns all feeds that support the specified [podcast namespace](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md) tag.   The only supported tags are:   - `podcast:value` using the `podcast-value` parameter   - `podcast:valueTimeSplit` using the `podcast-valueTimeSplit` parameter   Only the `podcast-value` or `podcast-valueTimeSplit` parameter should be used. If multiple are specified, the first parameter is used and the others are ignored.   When called without a `start_at` value, the top 500 feeds sorted by popularity are returned in descending order.   When called with a `start_at` value, the feeds are returned sorted by the `feedId` starting with the specified value up to the max number of feeds to return. The `nextStartAt` specifies the value to pass to the next `start_at`. Repeat this sequence until no items are returned.   Examples:   - https://api.podcastindex.org/api/1.0/podcasts/bytag?podcast-value&max=200&pretty   - https://api.podcastindex.org/api/1.0/podcasts/bytag?podcast-value&max=200&start_at=1&pretty   - https://api.podcastindex.org/api/1.0/podcasts/bytag?podcast-valueTimeSplit&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**podcast_value** | Option<**bool**> | Get feeds supporting the [`podcast:value`](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#value) tag.   Parameter shall not have a value  |  |
**podcast_value_time_split** | Option<**bool**> | Get feeds supporting the [`podcast:valueTimeSplit`](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#value-time-split) tag.   Parameter shall not have a value  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**start_at** | Option<**i32**> | Feed ID to start at for request  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsBytag200Response**](podcasts_bytag_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_dead

> models::PodcastsDead200Response podcasts_slash_dead(pretty)
Dead

This call returns all feeds that have been marked dead (`dead` == 1)   Dead feeds can also be accessed from object storage at https://public.podcastindex.org/podcastindex_dead_feeds.csv   Example: https://api.podcastindex.org/api/1.0/podcasts/dead?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsDead200Response**](podcasts_dead_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## podcasts_slash_trending

> models::PodcastsTrending200Response podcasts_slash_trending(max, since, lang, cat, notcat, pretty)
Trending

This call returns the podcasts/feeds that in the index that are trending.   Example: https://api.podcastindex.org/api/1.0/podcasts/trending?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**lang** | Option<**String**> | Specifying a language code (like \"en\") will return only episodes having that specific language.   You can specify multiple languages by separating them with commas.   If you also want to return episodes that have no language given, use the token \"unknown\". (ex. en,es,ja,unknown).   Values are not case sensitive.  |  |
**cat** | Option<**String**> | Use this argument to specify that you **ONLY** want episodes with these categories in the results.   Separate multiple categories with commas.   You may specify either the Category ID and/or the Category Name.   Values are not case sensitive.   The `cat` and `notcat` filters can be used together to fine tune a very specific result set.   Category numbers and names can be found in the [Podcast Namespace documentation](https://github.com/Podcastindex-org/podcast-namespace/blob/main/categories.json)  |  |
**notcat** | Option<**String**> | Use this argument to specify categories of episodes to **NOT** show in the results.   Separate multiple categories with commas.   You may specify either the Category ID and/or the Category Name.   Values are not case sensitive.   The `cat` and `notcat` filters can be used together to fine tune a very specific result set.   Category numbers and names can be found in the [Podcast Namespace documentation](https://github.com/Podcastindex-org/podcast-namespace/blob/main/categories.json)  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::PodcastsTrending200Response**](podcasts_trending_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


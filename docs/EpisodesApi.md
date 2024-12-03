# \EpisodesApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**episodes_slash_byfeedid**](EpisodesApi.md#episodes_slash_byfeedid) | **GET** /episodes/byfeedid | By Feed ID
[**episodes_slash_byfeedurl**](EpisodesApi.md#episodes_slash_byfeedurl) | **GET** /episodes/byfeedurl | By Feed URL
[**episodes_slash_byguid**](EpisodesApi.md#episodes_slash_byguid) | **GET** /episodes/byguid | By GUID
[**episodes_slash_byid**](EpisodesApi.md#episodes_slash_byid) | **GET** /episodes/byid | By ID
[**episodes_slash_byitunesid**](EpisodesApi.md#episodes_slash_byitunesid) | **GET** /episodes/byitunesid | By iTunes ID
[**episodes_slash_bypodcastguid**](EpisodesApi.md#episodes_slash_bypodcastguid) | **GET** /episodes/bypodcastguid | By Podcast GUID
[**episodes_slash_live**](EpisodesApi.md#episodes_slash_live) | **GET** /episodes/live | Live
[**episodes_slash_random**](EpisodesApi.md#episodes_slash_random) | **GET** /episodes/random | Random



## episodes_slash_byfeedid

> models::EpisodesByfeedid200Response episodes_slash_byfeedid(id, since, max, enclosure, fulltext, pretty)
By Feed ID

This call returns all the episodes we know about for this feed from the PodcastIndex ID. Episodes are in reverse chronological order.   When using the `enclosure` parameter, only the episode matching the URL is returned.   Examples:    - https://api.podcastindex.org/api/1.0/episodes/byfeedid?id=75075&pretty   - https://api.podcastindex.org/api/1.0/episodes/byfeedid?id=41504,920666&pretty   - Includes `persons`: https://api.podcastindex.org/api/1.0/episodes/byfeedid?id=169991&pretty   - Includes `value`: https://api.podcastindex.org/api/1.0/episodes/byfeedid?id=4058673&pretty   - Using `enclosure`: https://api.podcastindex.org/api/1.0/episodes/byfeedid?id=41504&enclosure=https://op3.dev/e/mp3s.nashownotes.com/NA-1551-2023-04-30-Final.mp3&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The PodcastIndex Feed ID or IDs to search for.   If searching for multiple IDs, separate values with a comma. A maximum of 200 IDs can be provided.  | [required] |
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**enclosure** | Option<**String**> | The URL for the episode enclosure to get the information for.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesByfeedid200Response**](episodes_byfeedid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_byfeedurl

> models::EpisodesByfeedurl200Response episodes_slash_byfeedurl(url, since, max, fulltext, pretty)
By Feed URL

This call returns all the episodes we know about for this feed from the feed URL. Episodes are in reverse chronological order.   Examples:    - https://api.podcastindex.org/api/1.0/episodes/byfeedurl?url=https://feeds.theincomparable.com/batmanuniversity&pretty   - Includes `persons`: https://api.podcastindex.org/api/1.0/episodes/byfeedurl?url=https://engineered.network/pragmatic/feed/index.xml&pretty   - Includes `value`: https://api.podcastindex.org/api/1.0/episodes/byfeedurl?url=https://closing-the-loop.github.io/feed.xml&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Podcast feed URL  | [required] |
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesByfeedurl200Response**](episodes_byfeedurl_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_byguid

> models::EpisodesByguid200Response episodes_slash_byguid(guid, feedurl, feedid, podcastguid, fulltext, pretty)
By GUID

Get all the metadata for a single episode by passing its guid and the feed id or URL.   The `feedid`, `feedurl`, or `podcastguid` is required.   Examples:     - Search using Podcast Index feed ID: https://api.podcastindex.org/api/1.0/episodes/byguid?guid=PC2084&feedid=920666&pretty   - Search using feed URL: https://api.podcastindex.org/api/1.0/episodes/byguid?guid=PC2084&feedurl=http://mp3s.nashownotes.com/pc20rss.xml&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guid** | **String** | The guid value for the episode to retrieve.     This value is the value specified in the feed's `<guid>` field.  | [required] |
**feedurl** | Option<**String**> | The Feed URL  |  |
**feedid** | Option<**String**> | The PodcastIndex Feed ID  |  |
**podcastguid** | Option<**String**> | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesByguid200Response**](episodes_byguid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_byid

> models::EpisodesByid200Response episodes_slash_byid(id, fulltext, pretty)
By ID

Get all the metadata for a single episode by passing its id.   Example: https://api.podcastindex.org/api/1.0/episodes/byid?id=16795090&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The PodcastIndex episode ID to search for.  | [required] |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesByid200Response**](episodes_byid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_byitunesid

> models::EpisodesByitunesid200Response episodes_slash_byitunesid(id, since, max, enclosure, fulltext, pretty)
By iTunes ID

This call returns all the episodes we know about for this feed from the iTunes ID. Episodes are in reverse chronological order.   When using the `enclosure` parameter, only the episode matching the URL is returned.   Examples:    - https://api.podcastindex.org/api/1.0/episodes/byitunesid?id=1441923632&pretty   - Using `enclosure`: https://api.podcastindex.org/api/1.0/episodes/byitunesid?id=269169796&enclosure=https://op3.dev/e/mp3s.nashownotes.com/NA-1551-2023-04-30-Final.mp3&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The iTunes Feed ID to search for  | [required] |
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**enclosure** | Option<**String**> | The URL for the episode enclosure to get the information for.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesByitunesid200Response**](episodes_byitunesid_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_bypodcastguid

> models::EpisodesByfeedurl200Response episodes_slash_bypodcastguid(guid, since, max, fulltext, pretty)
By Podcast GUID

This call returns all the episodes we know about for this feed from the [Podcast GUID](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid). Episodes are in reverse chronological order.   Example: https://api.podcastindex.org/api/1.0/episodes/bypodcastguid?guid=856cd618-7f34-57ea-9b84-3600f1f65e7f&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guid** | **String** | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [required] |
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesByfeedurl200Response**](episodes_byfeedurl_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_live

> models::EpisodesLive200Response episodes_slash_live(max, pretty)
Live

Get all episodes that have been found in the [podcast:liveitem](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#live-item) from the feeds.   Examples:     - https://api.podcastindex.org/api/1.0/episodes/live?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesLive200Response**](episodes_live_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## episodes_slash_random

> models::EpisodesRandom200Response episodes_slash_random(max, lang, cat, notcat, fulltext, pretty)
Random

This call returns a random batch of episodes, in no specific order.   Examples:    - https://api.podcastindex.org/api/1.0/episodes/random?notcat=News,Religion&lang=en,es&pretty   - https://api.podcastindex.org/api/1.0/episodes/random?max=2&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |[default to 1]
**lang** | Option<**String**> | Specifying a language code (like \"en\") will return only episodes having that specific language.   You can specify multiple languages by separating them with commas.   If you also want to return episodes that have no language given, use the token \"unknown\". (ex. en,es,ja,unknown).   Values are not case sensitive.  |  |
**cat** | Option<**String**> | Use this argument to specify that you **ONLY** want episodes with these categories in the results.   Separate multiple categories with commas.   You may specify either the Category ID and/or the Category Name.   Values are not case sensitive.   The `cat` and `notcat` filters can be used together to fine tune a very specific result set.   Category numbers and names can be found in the [Podcast Namespace documentation](https://github.com/Podcastindex-org/podcast-namespace/blob/main/categories.json)  |  |
**notcat** | Option<**String**> | Use this argument to specify categories of episodes to **NOT** show in the results.   Separate multiple categories with commas.   You may specify either the Category ID and/or the Category Name.   Values are not case sensitive.   The `cat` and `notcat` filters can be used together to fine tune a very specific result set.   Category numbers and names can be found in the [Podcast Namespace documentation](https://github.com/Podcastindex-org/podcast-namespace/blob/main/categories.json)  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::EpisodesRandom200Response**](episodes_random_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


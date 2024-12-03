/*
 * PodcastIndex.org API
 *
 * Download the openapi spec in [json](pi_api.json) or [yaml](pi_api.yaml) format.  # Overview  The Podcast Index (Podcast Index LLC) is a software developer focused partnership that provides tools and data to anyone who aspires to create new and exciting Podcast experiences without the heavy lifting of indexing, aggregation and data management.  # Example Code  Users have provided example code for working the API in the following languages:    - [AWS Lambda - python](https://github.com/tbowers/python-podcastindex-org-lambda)   - [Bash](https://github.com/suorcd/Bash-podcastindex-org-example)   - [C#](https://github.com/ComicStrip/csharp-podcastindex-org-example)   - [Elisp](https://github.com/sabexx/elisp-example)   - [Flutter/Dart](https://github.com/crediblecreative/flutter-dart-podcastindex-org-example)   - [Go](https://github.com/ComicStrip/Go-podcastindex-org-example)   - [Go](https://github.com/kilobit/podcast-index-client)   - [Java](https://github.com/ComicStrip/Java-podcastindex-org-example)   - [Java](https://github.com/stucoates/PodcastIndexJavaClient)   - [Node.js](https://github.com/ComicStrip/node.js-podcastindex-org-example)   - [PHP](https://github.com/Podcastindex-org/example-code)   - [Python](https://github.com/tbowers/python-podcastindex-org-example)   - [Swift](https://github.com/ComicStrip/Swift-podcastindex-org-example)   - ... [More](https://github.com/Podcastindex-org-Examples)  Don't see your desired language, create an example repo and create a [Pull Request](https://github.com/Podcastindex-org/docs-api/pulls) with a link to your example code project!  # Libraries  User created libraries for working with the API:    - Java       - [podcast4j](https://github.com/yusufyilmazfr/podcast4j)   - .NET       - [PodcastIndexSharp](https://www.nuget.org/packages/PodcastIndexSharp)   - Node.js/npm/yarn       - [podcast-index-api](https://www.npmjs.com/package/podcast-index-api)       - [podcastdx-client](https://www.npmjs.com/package/podcastdx-client)   - PHP       - [podcastindex-php](https://github.com/LowSociety/podcastindex-php)   - Python/pip       - [python-podcastindex](https://pypi.org/project/python-podcastindex/)   - R       - [podindexr](https://github.com/rpodcast/podindexr)   - Ruby       - [podcast-index](https://github.com/jasonyork/podcast-index)   - Swift       - [PodcastIndexKit](https://github.com/SparrowTek/PodcastIndexKit)   - Kotlin       - [PodcastIndex-SDK](https://github.com/mr3y-the-programmer/PodcastIndex-SDK)  Are we missing a library? Did you create one for a different language? Create a [Pull Request](https://github.com/Podcastindex-org/docs-api/pulls) with a link to the new library!  # Postman  A collection file for use in the [Postman](https://www.postman.com/) application is available for this API.    1. Download the contents of the [Postman Docs](https://github.com/Podcastindex-org/docs-api/tree/master/Postman%20Docs) folder.   2. Import the `PodcastIndex.postman_collection.json` collection to Postman   3. Import the `PodcastIndexOrgEnvironment.postman_environment.json` to Postman   4. Click \"Environments\" on the left sidebar   5. Select the checkbox next to the PodcastIndexOrgEnvironment entry   6. Set `AuthKey` and `SeceretKey` values under the \"Current Value\" column using your API information    7. Click \"Collections\" from the sidebar   8. Select PodcastIndex   9. Select and run the endpoint to test  # Contributing  The source for this API documentation is available at [https://github.com/Podcastindex-org/docs-api](https://github.com/Podcastindex-org/docs-api). Submit an Issue or create a Pull Request.  # Authentication Details  Sending an API request is easy. We use an Amazon-style request authorization token to secure each request.   Register for a free API key at https://api.podcastindex.org/   These headers parameters are required for each request: `User-Agent`, `X-Auth-Date`, `X-Auth-Key`, `Authorization`   See [Authentication](#auth) for description of parameters.  # Legal  Legal    - [Privacy Policy](https://github.com/Podcastindex-org/legal/blob/main/PrivacyPolicy.md)   - [Terms of Service](https://github.com/Podcastindex-org/legal/blob/main/TermsOfService.md)   - [License](https://github.com/Podcastindex-org/docs-api/blob/master/LICENSE)
 *
 * The version of the OpenAPI document: 1.12.1
 * Contact: info@podcastindex.org
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Stats : An array statistic properties
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// Total podcast feeds in the index.
    #[serde(rename = "feedCountTotal", skip_serializing_if = "Option::is_none")]
    pub feed_count_total: Option<i32>,
    /// Total individual podcast episodes in the index.
    #[serde(rename = "episodeCountTotal", skip_serializing_if = "Option::is_none")]
    pub episode_count_total: Option<i32>,
    /// Podcast feeds with a new episode released in the last 3 days.
    #[serde(
        rename = "feedsWithNewEpisodes3days",
        skip_serializing_if = "Option::is_none"
    )]
    pub feeds_with_new_episodes3days: Option<i32>,
    /// Podcast feeds with a new episode released in the last 10 days.
    #[serde(
        rename = "feedsWithNewEpisodes10days",
        skip_serializing_if = "Option::is_none"
    )]
    pub feeds_with_new_episodes10days: Option<i32>,
    /// Podcast feeds with a new episode released in the last 30 days.
    #[serde(
        rename = "feedsWithNewEpisodes30days",
        skip_serializing_if = "Option::is_none"
    )]
    pub feeds_with_new_episodes30days: Option<i32>,
    /// Podcast feeds with a new episode released in the last 90 days.
    #[serde(
        rename = "feedsWithNewEpisodes90days",
        skip_serializing_if = "Option::is_none"
    )]
    pub feeds_with_new_episodes90days: Option<i32>,
    /// Podcast feeds with a value block
    #[serde(
        rename = "feedsWithValueBlocks",
        skip_serializing_if = "Option::is_none"
    )]
    pub feeds_with_value_blocks: Option<i32>,
}

impl Stats {
    /// An array statistic properties
    pub fn new() -> Stats {
        Stats {
            feed_count_total: None,
            episode_count_total: None,
            feeds_with_new_episodes3days: None,
            feeds_with_new_episodes10days: None,
            feeds_with_new_episodes30days: None,
            feeds_with_new_episodes90days: None,
            feeds_with_value_blocks: None,
        }
    }
}

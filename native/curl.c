#include <curl/curl.h>

typedef void (*OnDataReceived)(const char *buffer, size_t buffer_size, void *user_data);

typedef struct GetRequestState
{
    OnDataReceived cb;
    void *user_data;
} GetRequestState;

static size_t write_callback(char *buffer, size_t size, size_t nmemb, void *user_data);

int send_get_request(const char *url, OnDataReceived cb, void *user_data)
{
    CURL *curl = curl_easy_init();
    CURLcode ret = CURLE_OK;
    GetRequestState state = {
        .user_data = user_data,
        .cb = cb,
    };

    if (curl)
    {
        curl_easy_setopt(curl, CURLOPT_URL, url);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &state);
        curl_easy_setopt(curl, CURLOPT_HEADER, 1L);
        ret = curl_easy_perform(curl);
        curl_easy_cleanup(curl);
    }
    else
    {
        ret = CURLE_FAILED_INIT;
    }

    return ret;
}

static size_t write_callback(char *buffer, size_t size, size_t nmemb, void *user_data)
{
    (void)size;
    GetRequestState *state = (GetRequestState *)user_data;

    // invoke the callback provided by the Rust caller
    (state->cb)(buffer, nmemb, state->user_data);

    // then tell curl we read everything
    return nmemb;
}

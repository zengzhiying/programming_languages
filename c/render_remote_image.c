#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>

// gcc render_remote_image.c -lcurl -o render

struct MemoryStruct {
  char *memory;
	size_t size;
};

static size_t WriteMemoryCallback(void *contents, size_t size, size_t nmemb,
		void *userp) {
	size_t realsize = size * nmemb;
	struct MemoryStruct *mem = (struct MemoryStruct *) userp;

	mem->memory = (char*)realloc(mem->memory, mem->size + realsize + 1);
	if (mem->memory == NULL) {
		/* out of memory! */
		printf("not enough memory (realloc returned NULL)\n");
		return 0;
	}

	memcpy(&(mem->memory[mem->size]), contents, realsize);
	mem->size += realsize;
	mem->memory[mem->size] = 0;

	return realsize;
}

int main(int argc, char *argv[]) {

	const char *image_url = "http://192.168.6.9:8980/abc.jpg";

	CURL *curl_handle;
	CURLcode res;

	struct MemoryStruct chunk;

	chunk.memory = (char*)malloc(1); /* will be grown as needed by the realloc above */
	chunk.size = 0; /* no data at this point */

	curl_global_init(CURL_GLOBAL_ALL);

	/* init the curl session */
	curl_handle = curl_easy_init();

	/* specify URL to get
	 * Note: be sure to make the following URL link to a valid image file
	 */
	curl_easy_setopt(curl_handle, CURLOPT_URL, image_url);

	/* send all data to this function  */
	curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, WriteMemoryCallback);

	/* we pass our 'chunk' struct to the callback function */
	curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void * )&chunk);

	/* some servers don't like requests that are made without a user-agent
	 field, so we provide one */
	curl_easy_setopt(curl_handle, CURLOPT_USERAGENT, "libcurl-agent/1.0");

	/* get it! */
	res = curl_easy_perform(curl_handle);

	/* check for errors */
	if (res != CURLE_OK) {
		fprintf(stderr, "curl_easy_perform() failed: %s\n",
				curl_easy_strerror(res));
	} else {
		/*
		 * Now, our chunk.memory points to a memory block that is chunk.size
		 * bytes big and contains the remote file.
		 *
		 * Do something nice with it!
		 *
		 * You should be aware of the fact that at this point we might have an
		 * allocated data block, and nothing has yet deallocated that data. So when
		 * you're done with it, you should free() it as a nice application.
		 */

		printf("%lu bytes retrieved\n", (long) chunk.size);
	}

	/* cleanup curl stuff */
	curl_easy_cleanup(curl_handle);

	if (chunk.memory) {
		free(chunk.memory);
	}

	/* we're done with libcurl, so clean it up */
	curl_global_cleanup();

	return 0;
}

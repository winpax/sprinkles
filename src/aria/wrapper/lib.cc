#include <iostream>

#include <aria2/aria2.h>

#include "lib.h"

int init() { return aria2::libraryInit(); }

int deinit() { return aria2::libraryDeinit(); }

int downloadEventCallback(aria2::Session *session, aria2::DownloadEvent event,
                          const aria2::A2Gid &gid, void *userData) {
  switch (event) {
  case aria2::EVENT_ON_DOWNLOAD_COMPLETE:
    std::cerr << "COMPLETE";
    break;
  case aria2::EVENT_ON_DOWNLOAD_ERROR:
    std::cerr << "ERROR";
    break;
  default:
    return 0;
  }
  //   std::cerr << " [" << aria2::gidToHex(gid) << "] ";
}

void *download(rust_aria2_config rust_config) {
  aria2::SessionConfig config;

  config.keepRunning = rust_config.keepRunning;
  config.useSignalHandler = rust_config.useSignalHandler;
  config.userData = rust_config.userData;
  config.downloadEventCallback = downloadEventCallback;

  aria2::Session *session = aria2::sessionNew(aria2::KeyVals(), config);
}

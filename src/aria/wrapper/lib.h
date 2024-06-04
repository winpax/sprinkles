
struct rust_aria2_config {
  bool keepRunning;
  bool useSignalHandler;
  void *userData;
};

int init();

int deinit();

void *download();

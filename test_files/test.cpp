#include <array>

#define pi 3.14

template <typename T> T add(T a, T b) { return a + b; }

int global = 1;

int main() {

  std::array<int, 10> arr;
  int local = 0;
  for (int i = 0; i < 10; i++) {
    arr[i] = i;
  };

  for (int i : arr) {
    local = add(local, i);
  }

  while (local < 100) {
    local = add(global, local);
  }

  do {
    local = add(global, local);
  } while (local < 1000);
  return 0;
}

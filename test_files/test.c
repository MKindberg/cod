int add(int a, int b) { return a + b; }

int global = 1;

int main() {

  int arr[10];
  int local = 0;
  for (int i = 0; i < 10; i++) {
    arr[i] = i;
  }

  while (local < 100) {
    local = add(global, local);
  }

  do {
    local = add(global, local);
  } while (local < 1000);

  return 0;
}

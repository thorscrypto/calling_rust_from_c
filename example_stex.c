#include <stdio.h>
#include <stdlib.h>

typedef struct Point {
  double x;
  double y;
};

struct Point move_point(struct Point p, double x_diff, double y_diff);
void move_point_inplace(struct Point *p, double x_diff, double y_diff);

int main() {
  struct Point p;
  p.x = 5.0;
  p.y = 1.0;
  struct Point p2 = move_point(p, 1.0, 3.0);
  printf("Point(%f, %f)\n", p2.x, p2.y);
  move_point_inplace(&p2, -1.0, -3.0);
  printf("Point(%f, %f)\n", p2.x, p2.y);
  
  return 0;
}


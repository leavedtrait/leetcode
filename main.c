#include <stdio.h>

int main()
{
    int length;
int height;
int width;

    printf("enter length \n");
    scanf("%d ", length);
    printf("enter height \n");
    scanf("%d", height);
    printf("enter width \n");
    scanf("%d", width);

    int volume = (length * width * height);

    printf("volume of cuboid is equal to:  %d \n", volume);
    return 0;
}
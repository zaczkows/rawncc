unsigned getNumber(int i)
{
    struct {
        int number;
        unsigned result;
    } static const TRANSLATION[] = { {0, 22}, {1, 35}, {2, 46}};

    return TRANSLATION[i].result;
}

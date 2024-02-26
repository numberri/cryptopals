/*
Challenge:
     One of the 60-character strings in this file has been encrypted by single-character XOR.
     Find it.
     (Your code from #3 should help.) 
Go plan:
    This is annoying - however there is a good chance that when being
    XORed with a character it will have a most frequent character that
    isn't in the most common e, a, r, i, o, t, n, s
    Weird thing I noticed - there is a solid chance that the string with
    the least amount of spaces may be the one that is XORed.

    In a Rust newbie way, the most challenging thing here may be opening
    the file... >_<
*/
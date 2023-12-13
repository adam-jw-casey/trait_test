# trait_test

Provides a macro to derive a tested version of traits

WORK IN PROGRESS

# Can't Fix
- Although it is desirable to make it impossible to implement a tested trait without running the tests, there is no way to prevent manually implementing the trait, which would bypass the derived tests. The sealed trait pattern is tempting, but this makes it impossible to implement the trait at all.

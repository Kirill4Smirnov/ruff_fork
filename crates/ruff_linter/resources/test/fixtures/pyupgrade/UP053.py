import array
import array as a
from array import array as array_alias


class Dummy:
    def array(self, typecode, initializer):
        return typecode, initializer


dummy = Dummy()
typecode = "u"

array.array("u", "abc")
a.array("u", "abc")
array_alias("u", "abc")
array.array(typecode="u", initializer="abc")

array.array("w", "abc")
array.array("b", [1, 2, 3])
array.array(typecode, "abc")
array.array("u".upper(), "abc")
dummy.array("u", "abc")

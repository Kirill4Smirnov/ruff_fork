import tomllib
from tomllib import TOMLDecodeError


tomllib.TOMLDecodeError("msg")
tomllib.TOMLDecodeError("msg", "doc")
tomllib.TOMLDecodeError("msg", "doc", 1, "extra")
tomllib.TOMLDecodeError(msg="msg", doc="doc")
tomllib.TOMLDecodeError(pos=1)
tomllib.TOMLDecodeError("msg", pos=1)
tomllib.TOMLDecodeError("msg", doc="doc")

TOMLDecodeError("msg")

tomllib.TOMLDecodeError("msg", "doc", 1)
tomllib.TOMLDecodeError("msg", doc="doc", pos=1)
tomllib.TOMLDecodeError(msg="msg", doc="doc", pos=1)
tomllib.TOMLDecodeError("msg", "doc", pos=1)

args = ("msg", "doc", 1)
kwargs = {"msg": "msg", "doc": "doc", "pos": 1}

tomllib.TOMLDecodeError(*args)
tomllib.TOMLDecodeError(**kwargs)

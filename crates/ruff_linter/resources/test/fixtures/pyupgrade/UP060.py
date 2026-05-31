import pathlib
from os.path import isreserved
from pathlib import Path, PurePath
from pathlib import Path as P


path = Path("AUX")
pure_path: PurePath = PurePath("COM1")
other = "NUL"


def build_path() -> PurePath:
    return PurePath("LPT2")


PurePath("NUL").is_reserved()
pathlib.Path("CON").is_reserved()
P("PRN").is_reserved()
path.is_reserved()
pure_path.is_reserved()


def f(p: PurePath) -> bool:
    return p.is_reserved()


Path(
    "LPT1",  # comment
).is_reserved()


other.is_reserved()
build_path().is_reserved()
isreserved(Path("NUL"))

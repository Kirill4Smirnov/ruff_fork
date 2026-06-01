from pathlib import Path, PurePath
from pathlib import PurePath as PP


path = PurePath("a/b/c")
parts = ("a", "b")


path.relative_to("a", "b")
path.relative_to("a", "b", walk_up=True)
path.is_relative_to("a", "b")
PP("a/b/c").relative_to("a", "b")
Path("a/b/c").is_relative_to("a", "b")

path.relative_to(
    "a",
    "b",  # comment
)
path.relative_to("a", *parts)


def f(p: PurePath, q: Path) -> None:
    p.relative_to("a", "b")
    q.is_relative_to("a", "b")


class Wrapper:
    def relative_to(self, *other):
        return other


Wrapper().relative_to("a", "b")

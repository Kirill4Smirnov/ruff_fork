import mimetypes
from mimetypes import MimeTypes, guess_type
from mimetypes import guess_type as gt
from pathlib import Path, PurePath


path = Path("image.png")
db = mimetypes.MimeTypes()


mimetypes.guess_type(path)
guess_type(path, strict=False)
gt(PurePath("archive.tar.gz"))
mimetypes.MimeTypes().guess_type(path)
db.guess_type(path)


def f(db: MimeTypes, path: PurePath) -> None:
    db.guess_type(path)


mimetypes.guess_type("https://example.com/image.png")
mimetypes.guess_type("image.png")
db.guess_type("image.png")

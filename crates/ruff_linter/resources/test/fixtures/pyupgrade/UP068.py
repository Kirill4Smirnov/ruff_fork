import gzip
import io
import pathlib
import tempfile


gzip.GzipFile(fileobj=open("archive.gz", "wb"))
gzip.GzipFile(None, fileobj=open("archive.gz", "ab"))
gzip.GzipFile(fileobj=pathlib.Path("archive.gz").open("xb"))
gzip.GzipFile(fileobj=tempfile.NamedTemporaryFile())
gzip.GzipFile(fileobj=tempfile.SpooledTemporaryFile())

raw = open("archive.gz", "wb")
gzip.GzipFile(fileobj=raw)

tmp = tempfile.TemporaryFile()
gzip.GzipFile(fileobj=tmp)

gzip.GzipFile(fileobj=open("archive.gz", "rb"))
gzip.GzipFile(fileobj=pathlib.Path("archive.gz").open("rb"))
gzip.GzipFile(fileobj=tempfile.NamedTemporaryFile("rb"))
gzip.GzipFile(fileobj=io.BytesIO())
gzip.GzipFile("archive.gz")
gzip.GzipFile("archive.gz", "wb")
gzip.GzipFile(fileobj=open("archive.gz", "wb"), mode="wb")

args = (None, None, 9, open("archive.gz", "wb"))
gzip.GzipFile(*args)

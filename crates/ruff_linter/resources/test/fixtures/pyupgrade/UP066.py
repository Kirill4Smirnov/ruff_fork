import io
import pickle
from pickle import Pickler


pickler = pickle.Pickler(io.BytesIO())
pickler.fast = True
value = pickler.fast

pickler_alias = Pickler(io.BytesIO())
pickler_alias.fast = True

pickle.Pickler.fast
Pickler.fast

(pickle.Pickler(io.BytesIO())).fast


class CustomPickler(pickle.Pickler):
    fast


CustomPickler.fast


class Other:
    fast = True


other = Other()
other.fast

fast = 1

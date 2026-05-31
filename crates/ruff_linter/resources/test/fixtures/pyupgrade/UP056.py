import decimal
import decimal as d
from decimal import HAVE_THREADS
from decimal import HAVE_THREADS as HAVE_THREADS_ALIAS


class Dummy:
    HAVE_THREADS = False


dummy = Dummy()

decimal.HAVE_THREADS
d.HAVE_THREADS
HAVE_THREADS
HAVE_THREADS_ALIAS

decimal.HAVE_CONTEXTVAR
dummy.HAVE_THREADS
True

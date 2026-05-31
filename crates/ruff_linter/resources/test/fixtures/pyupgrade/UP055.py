import sys
import sys as s
from sys import last_traceback, last_type, last_value
from sys import last_traceback as last_traceback_alias
from sys import last_type as last_type_alias
from sys import last_value as last_value_alias


class Dummy:
    last_type = None
    last_value = None
    last_traceback = None


dummy = Dummy()

sys.last_type
sys.last_value
sys.last_traceback
s.last_type
s.last_value
s.last_traceback
last_type
last_value
last_traceback
last_type_alias
last_value_alias
last_traceback_alias

sys.last_exc
dummy.last_type
dummy.last_value
dummy.last_traceback

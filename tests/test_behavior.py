import os , sys
sys.path.append(os.getcwd())
import pytest
import time
from testlib import *
from utils import *

@pytest.mark.asyncio
async def test_speed():
    ...
"""
Fixer for StandardError -> Exception.
"""

from typing import ClassVar, Literal

from .. import fixer_base

class FixStandarderror(fixer_base.BaseFix):
    BM_compatible: ClassVar[Literal[True]]
    PATTERN: ClassVar[str]
    def transform(self, node, results): ...

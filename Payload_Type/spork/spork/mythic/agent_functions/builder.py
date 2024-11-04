import asyncio
import json
import os
import pathlib
import sys
import tempfile
import traceback
from shutil import copytree
from mythic_container.PayloadBuilder import (
    PayloadType,
    SupportedOS,
    #BuildParameter,
    #BuildParameterType,
    BuildResponse,
    BuildStatus,
)

# pylint: disable=too-many-locals,too-many-branches,too-many-statements


# Class defining information about the Thanatos payload
class spork(PayloadType):
    name = "spork"  # Name of the payload
    file_extension = "exe"  # default file extension to use when creating payloads
    author = "@smoke"  # authors

    # Platforms that spork supports
    supported_os = [SupportedOS.Windows]
    wrapper = False
    wrapped_payloads = []
    # Description of the payload in Mythic
    note = "Windows agent written in Rust"

    # Payload does not support dynamic loading
    supports_dynamic_loading = False
    mythic_encrypts = True
    
    # Supported C2 profiles for spork
    c2_profiles = ["http"]

    agent_path = pathlib.Path(".") / "spork" / "mythic"
    agent_code_path = pathlib.Path(".") / "spork" / "agent_code"
    agent_icon_path = agent_path / "agent_icon" / "spork.svg"

    # This function is called to build a new payload
    async def build(self) -> BuildResponse:
        # Setup a new build response object
        resp = BuildResponse(status=BuildStatus.Success)
        return resp

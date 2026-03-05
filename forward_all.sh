#!/bin/bash
ssh -f -N -L 5432:127.0.0.1:5432 nakata
ssh -f -N -L 1808:127.0.0.1:1808 nakata


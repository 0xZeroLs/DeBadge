#!/bin/bash

solana transfer 2g9K42Pt5y58cejTHFLhqoQWKDUcB3s3AnGESmV9ySBW 10000 --allow-unfunded-recipient

anchor build

anchor test --skip-local-validator
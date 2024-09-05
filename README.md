# elb-log-transform

Transforms [Amazon Elastic Load Balancer (ELB) access logs][elblog] into [JSONL (NDJSON)][ndjson] for ingestion into other systems.

## Usage

Let's assume you've downloaded a whole bunch of ELB logs from your ELB log bucket:
```bash
aws s3 sync s3://my-awesome-log-bucket/AWSLogs/123123123/elasticloadbalancing/eu-west-1/2024/08/ ./lorgs
```
They're .gz files, and there's a lot of them, so you can't just `gzcat` them all and pipe them onwards, because
system command line limits. This tool doesn't do globbing or handle decompression (at this point anyway).

Here's a pipeline to decompress and transform a whole bunch of files into a single compressed JSONL file.
We're using `pv` to show progress.

```bash
find ./lorgs -name '*.log.gz' -exec gzcat '{}' '+' | cargo run --release -- | pv | gzip -9 > logs.jsonl.gz
```

The `logs.jsonl.gz` file can now be readily processed in other tools such as [DuckDB][duckdb]:

```sql
select
    request_method,
    regexp_replace(
        regexp_replace(
            request_url, '[\da-f]{8}-([\da-f]{4}-){3}[\da-f]{12}', 'UUID'
        ),
        '\?.+$',
        ''
    ) as ux,
    count(*) n
from 'logs.jsonl.gz'
where ux like '%api/%' escape '$' and user_agent not ilike 'Mozilla%'
group by request_method, ux
order by n desc;
```

would give you a list of API endpoints and their usage counts, excluding requests from browsers.


[elblog]: https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-access-logs.html
[ndjson]: https://github.com/ndjson/ndjson-spec
[duckdb]: https://duckdb.org/

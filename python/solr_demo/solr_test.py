#!/usr/bin/env python
# -*- coding:utf-8 -*-
from SolrUtils import SolrUtils
su = SolrUtils("http://zzylsl:8983/solr/zzylsl")
query = "title:*"
param = {
    'start': 0,
    'rows': 100,
}
results = su.query_results(query, param)
print results.hits
for rs in results:
    print rs['id'].encode("utf-8")
    print rs['title']


document = {"id":"002","title":"zzy"}
documents = [document]
documents.append({"id":"003", "title":unicode("标题12", 'utf-8')})
documents.append({"id":"004", "title":u"这是标题"})
print su.add_documents(documents)

print su.add_document({"id":"005", "title":u"标题5"})




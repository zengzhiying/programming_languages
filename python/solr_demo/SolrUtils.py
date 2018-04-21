# coding=utf-8
import pysolr
class SolrUtils(object):
    """solr 操作工具"""
    def __init__(self, solr_url):
        # 设置超时时间 默认是60s 超时直接抛出异常
        self.solr = pysolr.Solr(solr_url, timeout=3600)

    def query_results(self, query, param={'start': 0, 'rows': 10}):
        try:
            results = self.solr.search(query, **param)
            return results
        except Exception, error_msg:
            print("query error: %s" % str(error_msg))
            return -1

    def add_documents(self, documents):
        try:
            self.solr.add(documents)
            # self.solr.commit()
            return 0
        except Exception, error_msg:
            print("add error: %s" % str(error_msg))
            return -1

    # 添加1条
    def add_document(self, doc):
        try:
            self.solr.add([doc])
            return 0
        except Exception, error_msg:
            print("add error: %s" % str(error_msg))
            return -1


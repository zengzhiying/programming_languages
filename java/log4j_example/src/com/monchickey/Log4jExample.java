package com.monchickey;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public class Log4jExample {

    private static final Logger logger = LogManager.getLogger(Log4jExample.class.getName());

    public static void main(String[] args) throws InterruptedException {

        logger.trace("trace");
        logger.debug("debug.");
        logger.info("info.");
        logger.warn("warning.");
        logger.error("error.");
        logger.fatal("fatal.");
        for(int i = 0; i < 1000; i++) {
            logger.info("message: " + i);
            Thread.sleep(10);
        }
    }
}

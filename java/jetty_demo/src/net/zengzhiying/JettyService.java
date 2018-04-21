package net.zengzhiying;

import org.eclipse.jetty.server.Server;

import net.zengzhiying.controllers.TestController;

public class JettyService {
	public static void main(String[] args) throws Exception {
		Server server = new Server(8989);
		server.setHandler(new TestController());
		server.start();
		server.join();
	}
}

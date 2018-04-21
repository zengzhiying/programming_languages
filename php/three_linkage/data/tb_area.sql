-- phpMyAdmin SQL Dump
-- version 4.3.11
-- http://www.phpmyadmin.net
--
-- Host: 127.0.0.1
-- Generation Time: 2015-08-06 10:17:25
-- 服务器版本： 5.6.24
-- PHP Version: 5.6.8

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;

--
-- Database: `devproject`
--

-- --------------------------------------------------------

--
-- 表的结构 `tb_area`
--

CREATE TABLE IF NOT EXISTS `tb_area` (
  `area_id` int(10) unsigned NOT NULL,
  `area_name` varchar(64) NOT NULL,
  `area_parent` int(11) NOT NULL
) ENGINE=InnoDB AUTO_INCREMENT=51 DEFAULT CHARSET=utf8;

--
-- 转存表中的数据 `tb_area`
--

INSERT INTO `tb_area` (`area_id`, `area_name`, `area_parent`) VALUES
(1, '北京', 0),
(2, '上海', 0),
(3, '天津', 0),
(4, '重庆', 0),
(5, '安徽', 0),
(6, '福建', 0),
(7, '山东', 0),
(8, '北京市', 1),
(9, '朝阳区', 8),
(10, '昌平区', 8),
(11, '东城区', 8),
(12, '上海市', 2),
(13, '浦东新区', 12),
(14, '虹口区', 12),
(15, '普陀区', 12),
(16, '天津市', 3),
(17, '红桥区', 16),
(18, '河北区', 16),
(19, '河东区', 16),
(20, '重庆市', 4),
(21, '南岸区', 20),
(22, '九龙坡区', 20),
(23, '沙坪坝区', 20),
(24, '合肥市', 5),
(25, '庐阳区', 24),
(26, '福州', 6),
(27, '晋安区', 26),
(28, '济南市', 7),
(29, '青岛市', 7),
(30, '菏泽市', 7),
(31, '泰安市', 7),
(32, '市中区', 28),
(33, '历下区', 28),
(34, '槐荫区', 28),
(35, '黄岛区', 29),
(36, '市南区', 29),
(37, '市北区', 29),
(38, '崂山区', 29),
(39, '城阳区', 29),
(40, '胶南市', 29),
(41, '李沧区', 29),
(42, '即墨市', 29),
(43, '平度市', 29),
(44, '胶州市', 29),
(45, '牡丹区', 30),
(46, '郓城县', 30),
(47, '鄄城县', 30),
(48, '岱岳区', 31),
(49, '泰山区', 31),
(50, '肥城市', 31);

--
-- Indexes for dumped tables
--

--
-- Indexes for table `tb_area`
--
ALTER TABLE `tb_area`
  ADD PRIMARY KEY (`area_id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `tb_area`
--
ALTER TABLE `tb_area`
  MODIFY `area_id` int(10) unsigned NOT NULL AUTO_INCREMENT,AUTO_INCREMENT=51;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;

-- Seed data for system email domains
-- Comprehensive list of common system email patterns from major platforms and services

-- Job Application/ATS Platforms
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('1', '*.greenhouse.io', 'Greenhouse'),
('2', '*.lever.co', 'Lever'),
('3', '*.workday.com', 'Workday'),
('4', '*.smartrecruiters.com', 'SmartRecruiters'),
('5', '*.jobvite.com', 'Jobvite'),
('6', '*.taleo.net', 'Taleo'),
('7', '*.icims.com', 'iCIMS'),
('8', '*.bamboohr.com', 'BambooHR'),
('9', '*.jazzhr.com', 'JazzHR'),
('10', '*.recruitee.com', 'Recruitee'),
('11', '*.comeet.co', 'Comeet'),
('12', '*.hired.com', 'Hired'),
('13', '*.workable.com', 'Workable'),
('14', '*.bamboohr.com', 'BambooHR'),
('15', '*.personio.com', 'Personio'),
('16', '*.zoho.com', 'Zoho Recruit'),
('17', '*.bullhorn.com', 'Bullhorn'),
('18', '*.jobdiva.com', 'JobDiva'),
('19', '*.ceipal.com', 'CEIPAL'),
('20', '*.jobadder.com', 'JobAdder');

-- Common noreply patterns (most common system email pattern)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('21', 'noreply.*', 'No Reply (generic)'),
('22', 'no-reply.*', 'No Reply (hyphenated)'),
('23', 'donotreply.*', 'Do Not Reply (generic)'),
('24', 'do-not-reply.*', 'Do Not Reply (hyphenated)'),
('25', 'noreply@*', 'No Reply (any domain)'),
('26', 'no-reply@*', 'No Reply (hyphenated, any domain)'),
('27', 'donotreply@*', 'Do Not Reply (any domain)'),
('28', 'do-not-reply@*', 'Do Not Reply (hyphenated, any domain)');

-- Email Marketing/Notification Services
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('29', '*.mailchimp.com', 'Mailchimp'),
('30', '*.sendgrid.net', 'SendGrid'),
('31', '*.mandrillapp.com', 'Mandrill'),
('32', '*.postmarkapp.com', 'Postmark'),
('33', '*.amazonses.com', 'Amazon SES'),
('34', '*.mailgun.org', 'Mailgun'),
('35', '*.sparkpostmail.com', 'SparkPost'),
('36', '*.mailgun.com', 'Mailgun'),
('37', '*.sendinblue.com', 'Sendinblue'),
('38', '*.constantcontact.com', 'Constant Contact'),
('39', '*.campaignmonitor.com', 'Campaign Monitor'),
('40', '*.getresponse.com', 'GetResponse'),
('41', '*.aweber.com', 'AWeber'),
('42', '*.mailerlite.com', 'MailerLite'),
('43', '*.convertkit.com', 'ConvertKit'),
('44', '*.drip.com', 'Drip'),
('45', '*.klaviyo.com', 'Klaviyo'),
('46', '*.omnisend.com', 'Omnisend'),
('47', '*.activecampaign.com', 'ActiveCampaign');

-- Job Board Platforms
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('48', '*.linkedin.com', 'LinkedIn'),
('49', '*.indeed.com', 'Indeed'),
('50', '*.glassdoor.com', 'Glassdoor'),
('51', '*.ziprecruiter.com', 'ZipRecruiter'),
('52', '*.monster.com', 'Monster'),
('53', '*.careerbuilder.com', 'CareerBuilder'),
('54', '*.dice.com', 'Dice'),
('55', '*.simplyhired.com', 'SimplyHired'),
('56', '*.snagajob.com', 'Snagajob'),
('57', '*.theladders.com', 'The Ladders'),
('58', '*.angel.co', 'AngelList'),
('59', '*.stackoverflow.com', 'Stack Overflow Jobs'),
('60', '*.github.com', 'GitHub Jobs');

-- Generic notification/automated email patterns
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('61', 'notifications.*', 'Notifications (generic)'),
('62', 'alerts.*', 'Alerts (generic)'),
('63', 'automated.*', 'Automated (generic)'),
('64', 'system.*', 'System (generic)'),
('65', 'support@*', 'Support (automated)'),
('66', 'hello@*', 'Hello (automated)'),
('67', 'info@*', 'Info (automated)'),
('68', 'mailer.*', 'Mailer (generic)'),
('69', 'newsletter.*', 'Newsletter (generic)'),
('70', 'updates.*', 'Updates (generic)');

-- Social Media Platforms (automated emails)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('71', '*.facebook.com', 'Facebook'),
('72', '*.twitter.com', 'Twitter/X'),
('73', '*.instagram.com', 'Instagram'),
('74', '*.pinterest.com', 'Pinterest'),
('75', '*.reddit.com', 'Reddit');

-- E-commerce Platforms (automated emails)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('76', '*.shopify.com', 'Shopify'),
('77', '*.bigcommerce.com', 'BigCommerce'),
('78', '*.woocommerce.com', 'WooCommerce'),
('79', '*.magento.com', 'Magento'),
('80', '*.squarespace.com', 'Squarespace');

-- Payment/Financial Services (automated notifications)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('81', '*.stripe.com', 'Stripe'),
('82', '*.paypal.com', 'PayPal'),
('83', '*.square.com', 'Square'),
('84', '*.venmo.com', 'Venmo'),
('85', '*.zelle.com', 'Zelle');

-- Cloud/Infrastructure Services (automated notifications)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('86', '*.aws.amazon.com', 'Amazon Web Services'),
('87', '*.googlecloud.com', 'Google Cloud'),
('88', '*.azure.com', 'Microsoft Azure'),
('89', '*.heroku.com', 'Heroku'),
('90', '*.digitalocean.com', 'DigitalOcean'),
('91', '*.cloudflare.com', 'Cloudflare'),
('92', '*.vercel.com', 'Vercel'),
('93', '*.netlify.com', 'Netlify');

-- Communication Platforms (automated notifications)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('94', '*.slack.com', 'Slack'),
('95', '*.microsoft.com', 'Microsoft'),
('96', '*.zoom.us', 'Zoom'),
('97', '*.teams.microsoft.com', 'Microsoft Teams'),
('98', '*.discord.com', 'Discord');

-- Project Management Tools (automated notifications)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('99', '*.asana.com', 'Asana'),
('100', '*.trello.com', 'Trello'),
('101', '*.monday.com', 'Monday.com'),
('102', '*.jira.com', 'Jira'),
('103', '*.basecamp.com', 'Basecamp'),
('104', '*.notion.so', 'Notion'),
('105', '*.clickup.com', 'ClickUp'),
('106', '*.airtable.com', 'Airtable');

-- CRM Platforms (automated notifications)
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('107', '*.salesforce.com', 'Salesforce'),
('108', '*.hubspot.com', 'HubSpot'),
('109', '*.pipedrive.com', 'Pipedrive'),
('110', '*.zendesk.com', 'Zendesk'),
('111', '*.intercom.com', 'Intercom'),
('112', '*.freshdesk.com', 'Freshdesk'),
('113', '*.helpscout.com', 'Help Scout');

-- Additional common patterns
INSERT OR IGNORE INTO system_email_domains (id, domain_pattern, name) VALUES
('114', '*.mail.*', 'Mail subdomain (generic)'),
('115', '*.email.*', 'Email subdomain (generic)'),
('116', '*.messaging.*', 'Messaging subdomain (generic)'),
('117', '*.notify.*', 'Notify subdomain (generic)'),
('118', '*.alert.*', 'Alert subdomain (generic)');

